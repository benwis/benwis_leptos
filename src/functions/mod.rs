#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod auth;
pub mod dark_mode;
pub mod post;
pub mod user;

use cfg_if::cfg_if;
use leptos::reactive_graph::owner::Owner;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;
        use leptos::prelude::*;
        use leptos::{component, IntoView, view, server, error::ServerFnError, context::use_context};
        use crate::functions::auth::AuthSession;

        pub fn pool() -> Result<SqlitePool, ServerFnError> {
            println!("{:?}", Owner::current());
            use_context::<SqlitePool>()
                .ok_or("Pool missing.")
                .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or("Auth session missing.")
                .map_err(|e| ServerFnError::ServerError(e.to_string()))
        }
}}
