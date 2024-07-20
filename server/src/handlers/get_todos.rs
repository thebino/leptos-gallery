use leptos::{server, ServerFnError, use_context};
use sqlx::SqlitePool;

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use futures::future::join_all;

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))?;

    Ok(join_all(
        sqlx::query_as::<_, SqlTodo>("SELECT * FROM todos")
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|todo: &SqlTodo| todo.clone().into_todo(&pool)),
    )
        .await)
}
