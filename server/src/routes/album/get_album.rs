use axum::body::Body;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use http::{header, StatusCode};
use tracing::error;
use walkdir::WalkDir;

use crate::AppState;

pub async fn get_album(
    State(state): State<AppState>,
    Path(album_code): Path<String>,
) -> impl IntoResponse {
    let path = state.root.join(album_code.clone());
    let photo_path = path.clone().join("photos");
    let mut files: Vec<String> = Vec::new();

    for entry in WalkDir::new(photo_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());

        files.push(f_name);
    }

    let json_body = serde_json::to_string(&files).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(json_body))
        .map_err(|_| error!("Fail to compose delete item response"))
        .unwrap()
}
