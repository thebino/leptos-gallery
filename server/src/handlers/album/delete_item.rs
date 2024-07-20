use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use tokio::fs;
use tracing::error;
use crate::AppState;

pub async fn delete_item(
    State(state): State<AppState>,
    Path(album_code): Path<String>,
    Path(item_id): Path<String>
) -> impl IntoResponse {
    let path = state.root.join(album_code.clone()).join(item_id.clone());
    fs::remove_file(path).await.expect("Fail to delete item from filesystem!");

    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(axum::body::Body::empty())
        .map_err(|_| error!("Fail to compose delete item response"))
        .unwrap()
}
