use crate::{errors::BenwisAppError, models::post::*};
use cfg_if::cfg_if;
use chrono::Duration;
use indexmap::IndexMap;
use leptos::*;
cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::functions::pool;
    use slug::slugify;
    use leptos_axum::redirect;
    use chrono::{NaiveDateTime, prelude::*};
    use crate::models::Post;
}}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPosts, "/api")]
pub async fn get_posts(num: Option<usize>) -> Result<Result<Posts, BenwisAppError>, ServerFnError> {
    // Get Posts out of Context
    let Some(posts) = use_context::<PostsContainer>() else {
        return Ok(Err(BenwisAppError::InternalServerError));
    };

    let reader = posts.0.read();

    // If there are no Posts, try to get some if we haven't checked in the last minute
    if reader.posts.len() == 0 && (Utc::now() - reader.last_checked) >= Duration::minutes(1) {
        println!("Refetching");
        let mut writer = posts.0.write();
        writer.fetch_posts_from_github().await?;
        writer.last_checked = Utc::now();

        // Sort Posts by created_at date in descending order
        writer
            .posts
            .sort_unstable_by(|a, b, c, d| d.created_at.partial_cmp(&b.created_at).unwrap());
    }

    let mut processed_posts = IndexMap::new();
    match num {
        Some(n) => reader.posts.iter().take(n).for_each(|(k, v)| {
            processed_posts.insert(k.to_owned(), v.to_owned());
        }),
        None => processed_posts = reader.posts.clone(),
    };

    let out = Posts {
        posts: processed_posts,
        last_checked: reader.last_checked.clone(),
    };
    Ok(Ok(out))
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPost, "/api")]
pub async fn get_post(slug: String) -> Result<Result<Option<Post>, BenwisAppError>, ServerFnError> {
    let Some(posts) = use_context::<PostsContainer>() else {
        return Err(ServerFnError::ServerError(
            "Failed to get Posts".to_string(),
        ));
    };

    let reader = posts.0.read();

    // If there are no Posts, try to get some if we haven't checked in the last minute
    if reader.posts.get(&slug).is_none()
        && (Utc::now() - reader.last_checked) >= Duration::minutes(1)
    {
        println!("Fetching {slug}");
        let mut writer = posts.0.write();
        writer.fetch_post_from_github(&slug).await?;
        writer.last_checked = Utc::now();

        // Sort Posts by created_at date in descending order
        writer
            .posts
            .sort_unstable_by(|a, b, c, d| d.created_at.partial_cmp(&b.created_at).unwrap());
    }

    let post = match reader.posts.get(&slug) {
        Some(p) => Some(p.to_owned()),
        None => None,
    };

    Ok(Ok(post))
}
