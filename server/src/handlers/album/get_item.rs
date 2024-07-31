use std::path::PathBuf;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use http::{header, StatusCode};
use tokio::io::AsyncReadExt;
use tracing::error;

use crate::AppState;

pub async fn get_item(
    State(state): State<AppState>,
    Path(album_code): Path<String>,
    Path(item_id): Path<String>,
) -> impl IntoResponse {
    let path = state.root.join(album_code.clone()).join(item_id.clone());
    let path1 = path.clone();
    let path1 = PathBuf::from(path1);

    let filename = path1.file_name();
    let filename = match filename {
        Some(name) => name,
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(axum::body::Body::from("File name couldn't be determined".to_string()))
                .unwrap();
        }
    };
    let mut file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(axum::body::Body::from(format!("File not found: {}", err)))
                .unwrap();
        }
    };

    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
        // return Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap();
    // }
    // let stream = ReaderStream::new(file);

    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header(header::CONTENT_TYPE, mime::IMAGE_PNG.as_ref())
        .header(
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{:?}\"", filename),
        )
        .body(Body::from(buffer))
        .map_err(|_| error!("Fail to compose delete item response"))
        .unwrap()
}
