use crate::functions::auth;
use crate::models::{SafeUser, User};
use leptos::*;

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    Ok(auth.current_user)
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetSafeUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_safe_user(cx: Scope) -> Result<Option<SafeUser>, ServerFnError> {
    let auth = auth(cx)?;

    let safe_user = auth.current_user.map(|u| u.into());

    Ok(safe_user)
}
