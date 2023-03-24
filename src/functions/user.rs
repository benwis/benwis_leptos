use crate::functions::auth;
use crate::models::User;
use leptos::*;

#[server(GetUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    Ok(auth.current_user)
}
