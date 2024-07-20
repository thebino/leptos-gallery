use axum::extract::{Multipart, Path, State};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use http::StatusCode;
use tracing::{debug, error, info, warn};

use crate::AppState;

pub async fn add_item(
    State(state): State<AppState>,
    Path(album_code): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    dbg!(state.clone());
    dbg!(album_code.clone());

    let mut name: String = "".to_string();
    let mut bytes = Bytes::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "name" => {
                    name = field.text().await.unwrap();
                    debug!("name={}", name.clone());
                }
                "file" => {
                    bytes = field.bytes().await.unwrap();
                    debug!("{} bytes received", bytes.clone().len());
                }
                _ => continue,
            }
        }
    }

    debug!("{} bytes received", bytes.clone().len());

    let path = state.root.join(album_code.clone());
    let file_result = tokio::fs::write(&path.join(&name), &bytes).await;
    match file_result {
        Ok(_) => {
            info!("wrote to {}", path.to_str().unwrap().to_string());
        }
        Err(err) => {
            warn!(
                "Could not write file to path {}: {}",
                path.clone().to_str().unwrap().to_string(),
                err
            );
        }
    }

    Response::builder()
        .status(StatusCode::CREATED)
        .body(axum::body::Body::empty())
        .map_err(|_| error!("Fail to compose post item response"))
        .unwrap()
}
