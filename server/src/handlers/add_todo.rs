use leptos::{server, ServerFnError, use_context};
use sqlx::SqlitePool;

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let user = get_user().await?;
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    // TODO: remove fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Ok(sqlx::query(
        "INSERT INTO todos (title, user_id, completed) VALUES (?, ?, false)",
    )
        .bind(title)
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
}
