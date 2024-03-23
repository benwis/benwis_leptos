use crate::functions::auth;
use crate::models::{SafeUser, User};
use leptos::*;
#[cfg(feature = "ssr")]
use super::{auth::auth_session, pool};
use http::request::Parts;

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let con = pool()?;
    let req = expect_context::<Parts>();
    let user = match auth_session(&req, &con).await {
        Ok(u)=> Some(u),
        Err(_) => {
        //leptos_spin::redirect("/nedry");
        None
        }
    };

    Ok(user)
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetSafeUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_safe_user() -> Result<Option<SafeUser>, ServerFnError> {

    let con = pool()?;
    let req = expect_context::<Parts>();

    let safe_user = match auth_session(&req, &con).await {
        Ok(u)=> Some(u.into()),
        Err(_) => {
        //leptos_spin::redirect("/nedry");
        None
        }
    };
    Ok(safe_user)
}
