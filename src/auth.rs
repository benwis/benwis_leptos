use std::collections::HashSet;

use crate::error_template::{ErrorTemplate, ErrorTemplateProps};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::{Connection, SqliteConnection, SqlitePool};
    use axum_sessions_auth::{SessionSqlitePool, Authentication, HasPermission};
    use async_trait::async_trait;
    use bcrypt::{hash, verify, DEFAULT_COST};
    use crate::todo::db;

    pub type AuthSession = axum_sessions_auth::AuthSession<User, u32, SessionSqlitePool, SqlitePool>;
}}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub anonymous: bool,
    pub permissions: HashSet<String>,
}
impl Default for User {
    fn default() -> Self {
        let mut permissions = HashSet::new();

        permissions.insert("category:view".to_owned());

        Self {
            id: 1,
            anonymous: true,
            username: "Guest".into(),
            password: "".into(),
            permissions,
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {

    impl User {
        pub async fn get_user(id: u32, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = ?;",
            )
            .bind(id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[async_trait]
    impl Authentication<User, u32, SqlitePool> for User {
        async fn load_user(userid: u32, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get_user(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            !self.anonymous
        }

        fn is_active(&self) -> bool {
            !self.anonymous
        }

        fn is_anonymous(&self) -> bool {
            self.anonymous
        }
    }

    #[async_trait]
    impl HasPermission<SqlitePool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
            self.permissions.contains(perm)
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlUser {
        pub id: u32,
        pub anonymous: bool,
        pub username: String,
        pub password: String,
    }

    impl SqlUser {
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
            User {
                id: self.id,
                anonymous: self.anonymous,
                username: self.username,
                password: self.password,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            }
        }
    }
}
}

#[server(Foo, "/api")]
pub async fn foo() -> Result<String, ServerFnError> {
    Ok(String::from("Bar!"))
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let mut auth = use_context::<AuthSession>(cx)
        .ok_or("Auth session missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(Some(User::default()))
}

#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    todo!();

    // let user: Option<User> = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
    //     .bind(username)
    //     .fetch_optional(&mut conn)
    //     .await
    //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    // let mut auth = use_context::<AuthSession>(cx)
    //     .ok_or("Auth session missing.")
    //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    // let user = user
    //     .ok_or("User does not exist.")
    //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    // match verify(password, &user.password).map_err(|e| ServerFnError::ServerError(e.to_string()))? {
    //     true => todo!(),
    //     false => Err(ServerFnError::ServerError(
    //         "Password does not match.".to_string(),
    //     )),
    // }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String,
) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    match sqlx::query("INSERT INTO users (username, password) VALUES (?,?)")
        .bind(username)
        .bind(password_hashed)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let mut auth = use_context::<AuthSession>(cx)
        .ok_or("Auth session missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    todo!();
    // auth.logout().await;

    // Ok(())
}
