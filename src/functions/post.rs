use crate::{errors::BenwisAppError, models::post::*};
use cfg_if::cfg_if;
use leptos::prelude::*;
use leptos::{IntoView, component, prelude::*, server, view};
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::functions::{pool, auth};
    use slug::slugify;
    use leptos_axum::redirect;
    use chrono::{DateTime, NaiveDateTime, prelude::*};
    use futures::TryStreamExt;
}}

#[derive(Clone, Default, Debug, PartialEq, Params)]
pub struct PostQuery {
    pub p: Option<i64>,
}

fn parse_date_flexible(s: &str) -> Result<i64, BenwisAppError> {
    #[cfg(feature = "ssr")]
    {
        if let Ok(d) = DateTime::parse_from_rfc3339(s) {
            return Ok(d.timestamp());
        }
        if let Ok(d) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            return Ok(d.and_utc().timestamp());
        }
        Err(BenwisAppError::BadRequest(format!("Invalid date: {s}")))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = s;
        Ok(0)
    }
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(AddPost, "/api")]
pub async fn add_post(
    title: String,
    slug: String,
    created_at_pretty: String,
    excerpt: String,
    raw_content: String,
    content: String,
    toc: String,
    hero: String,
    hero_alt: String,
    hero_caption: String,
    tags: String,
    published: String,
    preview: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    // Redirect all non logged in users to Nedry!
    if auth.is_anonymous() {
        redirect("/nedry")
    }

    let published = published.parse::<bool>().unwrap_or(false);
    let preview = preview.parse::<bool>().unwrap_or(false);

    let user = super::user::get_user().await?;
    let slug = match slug.is_empty() {
        true => slugify(&title),
        false => slug,
    };

    let created_at = if created_at_pretty.is_empty() {
        Utc::now().timestamp()
    } else {
        parse_date_flexible(&created_at_pretty).map_err(|e| ServerFnError::new(e.to_string()))?
    };

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    let hero_opt = (!hero.is_empty()).then_some(hero);
    let hero_alt_opt = (!hero_alt.is_empty()).then_some(hero_alt);
    let hero_caption_opt = (!hero_caption.is_empty()).then_some(hero_caption);
    let toc_opt = (!toc.is_empty()).then_some(toc);
    let excerpt_opt = (!excerpt.is_empty()).then_some(excerpt);

    let tags_json = if tags.is_empty() {
        "[]".to_string()
    } else {
        // Accept either a JSON array or comma-separated
        if tags.trim_start().starts_with('[') {
            tags
        } else {
            let vec: Vec<String> = tags
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            serde_json::to_string(&vec).map_err(|e| ServerFnError::new(e.to_string()))?
        }
    };

    match sqlx::query(
        "INSERT INTO posts (title, slug, user_id, created_at, excerpt, raw_content, content, published, preview, hero, hero_alt, hero_caption, tags) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(slug)
    .bind(id)
    .bind(created_at)
    .bind(excerpt_opt)
    .bind(raw_content)
    .bind(content)
    .bind(published)
    .bind(preview)
    .bind(hero_opt)
    .bind(hero_alt_opt)
    .bind(hero_caption_opt)
    .bind(tags_json)
    .execute(&pool)
    .await
    {
        Ok(_row) => {
            let _ = toc_opt;
            Ok(())
        }
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    let pool = pool().unwrap();

    let mut posts = Vec::new();
    let mut rows =
        sqlx::query_as::<_, SqlPost>("SELECT id, user_id, title, slug, excerpt, raw_content, content, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts ORDER BY created_at DESC")
            .fetch(&pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        posts.push(row);
    }

    let mut converted_posts = Vec::with_capacity(posts.len());

    for t in posts {
        let post = t.into_post(&pool).await;
        converted_posts.push(post);
    }

    Ok(converted_posts)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPostsPaginated, "/api")]
pub async fn get_posts_paginated(page: i64, limit: i64) -> Result<Vec<Post>, ServerFnError> {
    let pool = pool()?;
    let page = if page <= 0 { 1 } else { page };
    let limit = if limit <= 0 { 5 } else { limit };
    let offset = (page - 1) * limit;

    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, SqlPost>(
        "SELECT id, user_id, title, slug, excerpt, raw_content, content, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts WHERE published = 1 ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        posts.push(row);
    }

    let mut converted_posts = Vec::with_capacity(posts.len());
    for t in posts {
        let post = t.into_post(&pool).await;
        converted_posts.push(post);
    }

    Ok(converted_posts)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPostCount, "/api")]
pub async fn get_post_count() -> Result<i64, ServerFnError> {
    let pool = pool()?;
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE published = 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(count)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetSomePosts, "/api")]
pub async fn get_some_posts() -> Result<Vec<Post>, ServerFnError> {
    let pool = pool()?;

    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, SqlPost>(
        "SELECT id, user_id, title, slug, excerpt, raw_content, content, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts ORDER by created_at DESC limit 3",
    )
    .fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        posts.push(row);
    }

    let mut converted_posts = Vec::with_capacity(posts.len());

    for t in posts {
        let post = t.into_post(&pool).await;
        converted_posts.push(post);
    }

    Ok(converted_posts)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetSomePostsMeta, "/api")]
pub async fn get_some_posts_meta() -> Result<Vec<PostMeta>, ServerFnError> {
    let pool = pool()?;

    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, SqlPostMeta>(
        "SELECT id, user_id, title, slug, excerpt, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts ORDER by created_at DESC limit 3",
    )
    .fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        posts.push(row);
    }

    let mut converted_posts = Vec::with_capacity(posts.len());

    for t in posts {
        let post = t.into_post_meta(&pool).await;
        converted_posts.push(post);
    }

    Ok(converted_posts)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPost, "/api")]
pub async fn get_post(slug: String) -> Result<Result<Option<Post>, BenwisAppError>, ServerFnError> {
    let pool = pool()?;

    let post = sqlx::query_as::<_, SqlPost>(
        "SELECT id, user_id, title, slug, excerpt, raw_content, content, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts WHERE slug=?",
    )
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

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPostWithSiblings, "/api")]
pub async fn get_post_with_siblings(
    slug: String,
) -> Result<Result<Option<PostTriad>, BenwisAppError>, ServerFnError> {
    let pool = pool()?;

    // Fetch current post
    let current = match sqlx::query_as::<_, SqlPost>(
        "SELECT id, user_id, title, slug, excerpt, raw_content, content, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts WHERE slug=?",
    )
    .bind(&slug)
    .fetch_one(&pool)
    .await
    {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) => return Ok(Ok(None)),
        Err(e) => return Ok(Err(e.into())),
    };

    let current_created_at: i64 =
        match sqlx::query_as::<_, (i64,)>("SELECT created_at FROM posts WHERE slug=?")
            .bind(&slug)
            .fetch_one(&pool)
            .await
        {
            Ok((ts,)) => ts,
            Err(e) => return Err(ServerFnError::new(e.to_string())),
        };

    // Previous = older post (smaller created_at)
    let previous_row = match sqlx::query_as::<_, SqlPost>(
        "SELECT id, user_id, title, slug, excerpt, raw_content, content, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts WHERE created_at < ? AND published = 1 ORDER BY created_at DESC LIMIT 1",
    )
    .bind(current_created_at)
    .fetch_optional(&pool)
    .await
    {
        Ok(r) => r,
        Err(e) => return Err(ServerFnError::new(e.to_string())),
    };

    // Next = newer post (larger created_at)
    let next_row = match sqlx::query_as::<_, SqlPost>(
        "SELECT id, user_id, title, slug, excerpt, raw_content, content, created_at, updated_at, published, preview, links, hero, hero_alt, hero_caption, tags FROM posts WHERE created_at > ? AND published = 1 ORDER BY created_at ASC LIMIT 1",
    )
    .bind(current_created_at)
    .fetch_optional(&pool)
    .await
    {
        Ok(r) => r,
        Err(e) => return Err(ServerFnError::new(e.to_string())),
    };

    let post = current.into_post(&pool).await;
    let previous = match previous_row {
        Some(p) => Some(p.into_post(&pool).await),
        None => None,
    };
    let next = match next_row {
        Some(p) => Some(p.into_post(&pool).await),
        None => None,
    };

    Ok(Ok(Some(PostTriad {
        previous,
        post,
        next,
    })))
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(UpdatePost, "/api")]
pub async fn update_post(
    id: i64,
    slug: String,
    title: String,
    hero: String,
    hero_alt: String,
    hero_caption: String,
    excerpt: String,
    created_at_pretty: String,
    raw_content: String,
    content: String,
    toc: String,
    tags: String,
    published: String,
    preview: String,
) -> Result<Result<bool, BenwisAppError>, ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    if auth.is_anonymous() {
        redirect("/nedry")
    }

    let published = published.parse::<bool>().unwrap_or(false);
    let preview = preview.parse::<bool>().unwrap_or(false);

    let created_at =
        parse_date_flexible(&created_at_pretty).map_err(|e| ServerFnError::new(e.to_string()))?;

    let slug = if slug.is_empty() {
        slugify(&title)
    } else {
        slug
    };

    let hero_opt = (!hero.is_empty()).then_some(hero);
    let hero_alt_opt = (!hero_alt.is_empty()).then_some(hero_alt);
    let hero_caption_opt = (!hero_caption.is_empty()).then_some(hero_caption);
    let toc_opt = (!toc.is_empty()).then_some(toc);
    let excerpt_opt = (!excerpt.is_empty()).then_some(excerpt);

    let tags_json = if tags.is_empty() {
        "[]".to_string()
    } else if tags.trim_start().starts_with('[') {
        tags
    } else {
        let vec: Vec<String> = tags
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        serde_json::to_string(&vec).map_err(|e| ServerFnError::new(e.to_string()))?
    };

    let post = sqlx::query(
        "UPDATE posts SET title=?, slug=?, hero=?, hero_alt=?, hero_caption=?, created_at=?, excerpt=?, raw_content=?, content=?, published=?, preview=?, tags=? WHERE id=?",
    )
    .bind(title)
    .bind(slug)
    .bind(hero_opt)
    .bind(hero_alt_opt)
    .bind(hero_caption_opt)
    .bind(created_at)
    .bind(excerpt_opt)
    .bind(raw_content)
    .bind(content)
    .bind(published)
    .bind(preview)
    .bind(tags_json)
    .bind(id)
    .execute(&pool)
    .await;
    let _ = toc_opt;
    let res = match post {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(e) => Err(e.into()),
    };
    Ok(res)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(DeletePost, "/api")]
pub async fn delete_post(id: u16) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    if auth.is_anonymous() {
        redirect("/nedry")
    }

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
}
