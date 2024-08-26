use axum::extract::{Multipart, Path, State};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use http::StatusCode;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor};
use thumbnailer::{create_thumbnails, ThumbnailSize};
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
    let photo_path = path.clone().join("photos");
    let thumb_path = path.clone().join("thumbs");
    fs::create_dir_all(photo_path.clone()).expect("Fail to create photo directory");
    fs::create_dir_all(thumb_path.clone()).expect("Fail to create thumb directory");
    let file_result = tokio::fs::write(&photo_path.join(&name), &bytes).await;
    match file_result {
        Ok(_) => {
            info!(
                "wrote photo to {}",
                photo_path.clone().join(&name).to_str().unwrap().to_string()
            );
        }
        Err(err) => {
            warn!(
                "Could not write photo file to path {}: {}",
                photo_path.clone().to_str().unwrap().to_string(),
                err
            );
        }
    }

    // create thumbnail
    let file = File::open(&photo_path.join(&name)).unwrap();
    let reader = BufReader::new(file);
    let mut thumbnails =
        create_thumbnails(reader, mime::IMAGE_JPEG, [ThumbnailSize::Medium]).unwrap();
    let thumbnail = thumbnails.pop().unwrap();
    let mut buf = Cursor::new(Vec::new());
    thumbnail.write_png(&mut buf).unwrap();
    let data = buf.into_inner();

    let file_result = tokio::fs::write(&thumb_path.join(&name), &data).await;
    match file_result {
        Ok(_) => {
            info!(
                "wrote thumb to {}",
                thumb_path.to_str().unwrap().to_string()
            );
        }
        Err(err) => {
            warn!(
                "Could not write thumb file to path {}: {}",
                thumb_path.clone().to_str().unwrap().to_string(),
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
