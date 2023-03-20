use leptos::*;
use crate::models::post::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::functions::pool;
    }}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope) -> Result<Vec<post>, ServerFnError> {
    use futures::TryStreamExt;
    let pool = pool(cx)?;

    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, Sqlpost>("SELECT * FROM posts").fetch(&pool);

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

    let posts: Vec<post> = converted_posts;

    Ok(posts)
}

#[server(AddPost, "/api")]
pub async fn add_post(cx: Scope, title: String) -> Result<(), ServerFnError> {
    let user = super::user::get_user(cx).await?;
    let pool = pool(cx)?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query("INSERT INTO posts (title, user_id, completed) VALUES (?, ?, false)")
        .bind(title)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
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
