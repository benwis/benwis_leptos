// use crate::{errors::BenwisAppError, models::post::*};
// use cfg_if::cfg_if;
// use chrono::Duration;
// use indexmap::IndexMap;
// use leptos::*;
// use tower_http::services::ServeFile;
// cfg_if! {
// if #[cfg(feature = "ssr")] {
//     use crate::functions::pool;
//     use slug::slugify;
//     use leptos_axum::{redirect, ResponseOptions};
//     use chrono::{NaiveDateTime, prelude::*};
//     use http::HeaderValue;
//     use crate::models::Post;
//     use serde_json::{json};
// }}
//
// #[server(UploadFiles, "/api", "Url", "upload")]
// pub async fn upload_files() -> Result<Result<(), BenwisAppError>, ServerFnError> {}
