use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use argon2::{
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };    

    use std::str::FromStr;
    use http::{header::SET_COOKIE, HeaderValue};
    use axum_extra::extract::CookieJar;
    use async_trait::async_trait;
    use crate::models::User;
    use crate::errors::BenwisAppError;
    use rand_core::OsRng;
    use http::request::Parts;
    use crate::session::SqliteStore;
    use crate::functions::pool;
    use async_session::{Session, SessionStore};

    /// Hash Argon2 password
    pub fn hash_password(password: &[u8]) -> Result<String, BenwisAppError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password, &salt)?.to_string();
        Ok(password_hash)
    }
    /// Verify Password
    pub fn verify_password(password: &str, password2: &str) -> Result<(), BenwisAppError> {
        let argon2 = Argon2::default();
        // Verify password against PHC string
        let parsed_hash = PasswordHash::new(&password)?;
        Ok(argon2.verify_password(password2.as_bytes(), &parsed_hash)?)
    }

    #[derive(sqlx::FromRow,Debug, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    /// Verify the user is who they say they are
    pub async fn auth_user(name: &str, password: &str, con: &SqlitePool) -> Result<User, BenwisAppError>{
        // Does the user exist
        let Some(user) = User::get_from_username(name, con).await else{
            return Err(BenwisAppError::AuthError);
        };

        // Check that password is correct
        match verify_password(password, &user.password){
            Ok(_) => Ok(user),
            Err(e) => {println!("Verify Failed: {e}"); Err(BenwisAppError::AuthError)},
        }
    }
    pub fn get_session_cookie_value(req_parts: &Parts)-> Result<Option<String>, BenwisAppError>{
    let cookie_jar = CookieJar::from_headers(&req_parts.headers);
    let session_cookie = match cookie_jar.get("benwis_session") {
                Some(c) => Some(c.value().to_owned()),
                None => None,
            };

    Ok(session_cookie)
    }

    pub async fn auth_session(req_parts: &Parts, con: &SqlitePool)-> Result<User, BenwisAppError>{
    
    let store = expect_context::<SqliteStore>();
    let session_val = match get_session_cookie_value(req_parts)?{
    Some(sv) => sv,
    None => return Err(BenwisAppError::AuthError),
    };

    let Some(session) = store.load_session(session_val).await? else{
        return Err(BenwisAppError::InternalServerError);
    }; 
    let Some(user_id) = session.get("user_id") else{
        return Err(BenwisAppError::AuthError);
    };

    let user = match User::get(user_id, con).await{
        Some(u) => u,
        None => return Err(BenwisAppError::AuthError)
    };  
    Ok(user)
    }

    /// Create a new Session and store User id in it
    pub async fn create_session(user_id: i64)-> Result<String, BenwisAppError>{
        let mut session = Session::new();
        session.insert("user_id", user_id)?;

        let session_store = expect_context::<SqliteStore>();
        let cookie_value = session_store.store_session(session).await?.unwrap();
        Ok(cookie_value)
    }

    /// Destroy the Session if it exists
    pub async fn logout_session(cookie_value: &str)-> Result<(), BenwisAppError>{
        let store = expect_context::<SqliteStore>();
        let session = match store.load_session(cookie_value.to_string()).await?{
            Some(s) =>s,
            None => return Ok(())
        };
        store.destroy_session(session).await?;
        Ok(())
    }

}
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let Some(parts) = use_context::<Parts>() else {
        return Ok(());
    };
    let con = pool()?;
    let user = auth_user(&username, &password, &con).await?;
    let session_cookie = create_session(user.id).await?;

    let res_options = expect_context::<leptos_axum::ResponseOptions>();
    let cookie_val = format!("benwis_session={session_cookie};Path=/;SameSite=Strict;");
    res_options.insert_header(SET_COOKIE, HeaderValue::from_str(&cookie_val).unwrap());
    leptos_axum::redirect("/");
    Ok(())
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    display_name: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;

    
    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }
    // Don't want anyone signing up but me!
    if username != "benwis" {
        leptos_axum::redirect("/nedry");
        return Ok(());
    }

    let password_hashed = hash_password(password.as_bytes()).unwrap();

    sqlx::query("INSERT INTO users (username, display_name, password) VALUES (?,?, ?)")
        .bind(username.clone())
        .bind(display_name.clone())
        .bind(password_hashed)
        .execute(&pool)
        .await?;

    let user = User::get_from_username(&username, &pool)
        .await
        .ok_or("Signup failed: User does not exist.")
        .map_err(ServerFnError::new)?;


    Ok(())
}


#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    println!("LOGGING OUT");
    let Some(parts) = use_context::<Parts>() else {
        return Ok(());
    };
    let con = pool()?;
    let Some(session) = get_session_cookie_value(&parts)? else{
        return Ok(());
    };
    logout_session(&session).await?;

    // Delete session cookie by expiring it
    let res_parts = expect_context::<leptos_axum::ResponseOptions>();
    res_parts.insert_header(SET_COOKIE, HeaderValue::from_static("benwis_session=no;Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;"));
    res_parts.insert_header(SET_COOKIE, HeaderValue::from_static("sessionid=no;Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;"));
    leptos_axum::redirect("/");

    Ok(())
}
