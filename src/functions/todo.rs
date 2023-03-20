use leptos::*;
use crate::models::todo::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::functions::pool;
    }}

#[server(GetTodos, "/api")]
pub async fn get_todos(cx: Scope) -> Result<Vec<Todo>, ServerFnError> {
    use futures::TryStreamExt;
    let pool = pool(cx)?;

    let mut todos = Vec::new();
    let mut rows = sqlx::query_as::<_, SqlTodo>("SELECT * FROM todos").fetch(&pool);

    while let Some(row) = rows
        .try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
    {
        todos.push(row);
    }

    // why can't we just have async closures?
    // let mut rows: Vec<Todo> = rows.iter().map(|t| async { t }).collect();

    let mut converted_todos = Vec::with_capacity(todos.len());

    for t in todos {
        let todo = t.into_todo(&pool).await;
        converted_todos.push(todo);
    }

    let todos: Vec<Todo> = converted_todos;

    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(cx: Scope, title: String) -> Result<(), ServerFnError> {
    let user = super::user::get_user(cx).await?;
    let pool = pool(cx)?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query("INSERT INTO todos (title, user_id, completed) VALUES (?, ?, false)")
        .bind(title)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(cx: Scope, id: u16) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;

    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
