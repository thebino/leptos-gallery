use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use http::{header, StatusCode};
use std::collections::HashMap;
use std::io::Read;
use tracing::error;

use crate::AppState;

pub async fn get_item(
    State(state): State<AppState>,
    Path((album_code, item_id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let width = params.get("width");
    let path = match width {
        Some(_) => state
            .root
            .join(album_code.clone())
            .join("thumbs")
            .join(item_id.clone()),
        None => state
            .root
            .join(album_code.clone())
            .join("photos")
            .join(item_id.clone()),
    };

    if path.exists() {
        let mut file = match std::fs::File::open(path.clone()) {
            Ok(file) => file,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(axum::body::Body::from(format!("File not found: {}", err)))
                    .unwrap()
            }
        };
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read image").into_response();
        };

        let filename = String::from(path.file_name().unwrap().to_string_lossy());
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime::IMAGE_JPEG.as_ref())
            .header(
                header::CONTENT_DISPOSITION,
                &format!("attachment; filename=\"{:?}\"", filename),
            )
            .body(buffer.into())
            .map_err(|_| error!("Fail to compose delete item response"))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::from("File not found[1]".to_string()))
            .unwrap()
    }
}
