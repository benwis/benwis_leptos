use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_sessions_auth::{SessionSqlitePool, Authentication, HasPermission};
    use argon2::{
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };    use crate::functions::{pool, auth};
    pub type AuthSession = axum_sessions_auth::AuthSession<User, i64, SessionSqlitePool, SqlitePool>;
    use async_trait::async_trait;
    use crate::models::User;
    use crate::errors::BenwisAppError;
    use rand_core::OsRng;

    /// Hash Argon2 password
    pub fn hash_password(password: &[u8]) -> Result<String, BenwisAppError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password, salt.as_ref())?.to_string();
        Ok(password_hash)
    }
    /// Verify Password
    pub fn verify_password(password: String, password2: String) -> Result<(), BenwisAppError> {
        let argon2 = Argon2::default();
        // Verify password against PHC string
        let parsed_hash = PasswordHash::new(&password)?;
        Ok(argon2.verify_password(password2.as_bytes(), &parsed_hash)?)
    }

    #[derive(sqlx::FromRow,Debug, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[async_trait]
    impl Authentication<User, i64, SqlitePool> for User {
        async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<SqlitePool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
            self.permissions.contains(perm)
        }
    }
}
}

#[tracing::instrument(level = "info", fields(error), ret,err)]
#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or("User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match verify_password(user.password, password)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
    {
        Ok(_) => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect(cx, "/");
            Ok(())
        }
        Err(_) => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[tracing::instrument(level = "info", fields(error), ret,err)]
#[server(Signup, "/api")]
pub async fn signup(
    cx: Scope,
    username: String,
    display_name: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }
    // Don't want anyone signing up but me!
    if username != "benwis" {
        leptos_axum::redirect(cx, "/nedry");
        return Ok(());
    }

    let password_hashed = hash_password(password.as_bytes()).unwrap();

    sqlx::query("INSERT INTO users (username, display_name, password) VALUES (?,?, ?)")
        .bind(username.clone())
        .bind(display_name.clone())
        .bind(password_hashed)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let user = User::get_from_username(username, &pool)
        .await
        .ok_or("Signup failed: User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect(cx, "/");

    Ok(())
}

#[tracing::instrument(level = "info", fields(error), ret,err)]
#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let auth = auth(cx)?;

    auth.logout_user();
    leptos_axum::redirect(cx, "/");

    Ok(())
}
