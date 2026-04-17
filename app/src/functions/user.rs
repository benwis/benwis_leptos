use crate::functions::auth;
use crate::models::{SafeUser, User};
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use leptos::{component, server, view, IntoView};

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = auth()?;

    Ok(auth.current_user)
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetSafeUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_safe_user() -> Result<Option<SafeUser>, ServerFnError> {
    let auth = auth()?;

    let safe_user = auth.current_user.map(|u| u.into());

    Ok(safe_user)
}
