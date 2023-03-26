use crate::{errors::BenwisAppError, models::post::*};
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::functions::pool;
    use slug::slugify;
}}

#[server(AddPost, "/api")]
pub async fn add_post(
    cx: Scope,
    title: String,
    slug: String,
    excerpt: String,
    content: String,
) -> Result<(), ServerFnError> {
    let user = super::user::get_user(cx).await?;
    let pool = pool(cx)?;
    let slug = match slug.is_empty() {
        true => slugify(&title),
        false => slug,
    };

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    match sqlx::query(
        "INSERT INTO posts (title, slug, user_id, excerpt, content) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(slug)
    .bind(id)
    .bind(excerpt)
    .bind(content)
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

    // why can't we just have async closures?
    // let mut rows: Vec<post> = rows.iter().map(|t| async { t }).collect();

    let mut converted_posts = Vec::with_capacity(posts.len());

    for t in posts {
        let post = t.into_post(&pool).await;
        converted_posts.push(post);
    }

    let posts: Vec<Post> = converted_posts;

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
    content: String,
    published: String,
    preview: String,
) -> Result<Result<bool, BenwisAppError>, ServerFnError> {
    let pool = pool(cx)?;

    let published = published.parse::<bool>().unwrap();
    let preview = preview.parse::<bool>().unwrap();


    let post = sqlx::query("UPDATE posts SET title=?, hero=?, excerpt=?, content=?,published=?,preview=? WHERE slug=?")
        .bind(title)
        .bind(hero)
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

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
