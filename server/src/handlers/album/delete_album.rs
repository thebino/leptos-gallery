use std::fs;
use axum::{http::StatusCode, response::Response};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use tracing::{error};
use crate::AppState;

pub async fn delete_album(
    State(state): State<AppState>,
    Path(album_code): Path<String>
) -> impl IntoResponse {
    dbg!(album_code.clone());
    let path = state.root.join(album_code.clone());
    fs::remove_dir_all(path).expect("Fail to delete album directory!");

    sqlx::query("DELETE FROM users WHERE albumcode = $1")
        .bind(album_code)
        .execute(&state.pool)
        .await
        .expect("Fail to delete album from database!");

    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(axum::body::Body::empty())
        .map_err(|_| error!("Fail to compose delete album response"))
        .unwrap()
}
