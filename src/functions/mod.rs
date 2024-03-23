#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod auth;
pub mod dark_mode;
pub mod post;
pub mod user;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;
        use leptos::*;

        pub fn pool() -> Result<SqlitePool, ServerFnError> {
            use_context::<SqlitePool>()
                .ok_or("Pool missing.")
                .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))
        }
}}
