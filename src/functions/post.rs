use crate::{errors::BenwisAppError, models::post::*};
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::functions::{pool, auth};
    use slug::slugify;
    use leptos_axum::redirect;
    use chrono::{NaiveDateTime, prelude::*};
}}

#[server(AddPost, "/api")]
pub async fn add_post(
    cx: Scope,
    title: String,
    slug: String,
    created_at_pretty: String,
    excerpt: String,
    content: String,
    published: String,
    preview: String,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    // Redirect all non logged in users to Nedry!
    if auth.is_anonymous() {
        redirect(cx, "/nedry")
    }

    let published = published.parse::<bool>().unwrap();
    let preview = preview.parse::<bool>().unwrap();

    let user = super::user::get_user(cx).await?;
    let slug = match slug.is_empty() {
        true => slugify(&title),
        false => slug,
    };
    //2023-03-28 19:30:41

    let created_at = match created_at_pretty.is_empty() {
        false => NaiveDateTime::parse_from_str(&created_at_pretty, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?
            .timestamp(),
        true => Utc::now().timestamp(),
    };

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    match sqlx::query(
        "INSERT INTO posts (title, slug, user_id, created_at, excerpt, content, published, preview) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(slug)
    .bind(id)
    .bind(created_at)
    .bind(excerpt)
    .bind(content)
    .bind(published)
    .bind(preview)
    .execute(&pool)
    .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope) -> Result<Vec<Post>, ServerFnError> {
    use futures::TryStreamExt;
    let pool = pool(cx)?;

    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, SqlPost>("SELECT * FROM posts").fetch(&pool);

    while let Some(row) = rows
        .try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
    {
        posts.push(row);
    }

    let mut converted_posts = Vec::with_capacity(posts.len());

    for t in posts {
        let post = t.into_post(&pool).await;
        converted_posts.push(post);
    }

    let mut posts: Vec<Post> = converted_posts;

    // Reverse the order of the posts
    posts.sort_unstable_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());

    Ok(posts)
}

#[server(GetSomePosts, "/api")]
pub async fn get_some_posts(cx: Scope) -> Result<Vec<Post>, ServerFnError> {
    use futures::TryStreamExt;
    let pool = pool(cx)?;

    let mut posts = Vec::new();
    let mut rows =
        sqlx::query_as::<_, SqlPost>("SELECT * FROM posts ORDER by created_at DESC limit 3")
            .fetch(&pool);

    while let Some(row) = rows
        .try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
    {
        posts.push(row);
    }

    let mut converted_posts = Vec::with_capacity(posts.len());

    for t in posts {
        let post = t.into_post(&pool).await;
        converted_posts.push(post);
    }

    let mut posts: Vec<Post> = converted_posts;

    // Reverse the order of the posts
    posts.sort_unstable_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());

    Ok(posts)
}
#[server(GetPost, "/api")]
pub async fn get_post(
    cx: Scope,
    slug: String,
) -> Result<Result<Option<Post>, BenwisAppError>, ServerFnError> {
    let pool = pool(cx)?;

    let post = sqlx::query_as::<_, SqlPost>("SELECT * FROM posts WHERE slug=?")
        .bind(slug)
        .fetch_one(&pool)
        .await;
    let post = match post {
        Ok(r) => Ok(Some(r.into_post(&pool).await)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(e) => Err(e),
    }
    .map_err(|e| e.into());
    Ok(post)
}
#[server(UpdatePost, "/api")]
pub async fn update_post(
    cx: Scope,
    slug: String,
    title: String,
    hero: String,
    excerpt: String,
    created_at_pretty: String,
    content: String,
    published: String,
    preview: String,
) -> Result<Result<bool, BenwisAppError>, ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    // Redirect all non logged in users to Nedry!
    if auth.is_anonymous() {
        redirect(cx, "/nedry")
    }

    let published = published.parse::<bool>().unwrap();
    let preview = preview.parse::<bool>().unwrap();
    //2023-03-28 19:30:41
    let created_at = NaiveDateTime::parse_from_str(&created_at_pretty, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let post = sqlx::query("UPDATE posts SET title=?, hero=?, created_at=?, excerpt=?, content=?,published=?,preview=? WHERE slug=?")
        .bind(title)
        .bind(hero)
        .bind(created_at.timestamp())
        .bind(excerpt)
        .bind(content)
        .bind(published)
        .bind(preview)
        .bind(slug)
        .execute(&pool)
        .await;
    let res = match post {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(e) => Err(e.into()),
    };
    Ok(res)
}

#[server(DeletePost, "/api")]
pub async fn delete_post(cx: Scope, id: u16) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    // Redirect all non logged in users to Nedry!
    if auth.is_anonymous() {
        redirect(cx, "/nedry")
    }

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
