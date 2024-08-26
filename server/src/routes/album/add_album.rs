use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{body, http::StatusCode, response::Response};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;
use tracing::error;

pub async fn add_album(State(state): State<AppState>, passcode: String) -> impl IntoResponse {
    let album_code = generate_album_code();
    let path = state.root.join(album_code.clone());
    dbg!(passcode.clone());
    fs::create_dir_all(path.clone()).expect("Fail to create album directory");

    sqlx::query("INSERT INTO users (albumcode, passcode) VALUES ($1, $2)")
        .bind(album_code.clone())
        .bind(passcode)
        .execute(&state.pool)
        .await
        .expect("Fail to insert new albumcode into database");

    Response::builder()
        .status(StatusCode::CREATED)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(http::header::LOCATION, format!("/album/{album_code}"))
        .body(body::Body::empty())
        .map_err(|_| error!("Fail to compose album response!"))
        .unwrap()
}

/// Generates a random string/number combination as access code for an album
fn generate_album_code() -> String {
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.sample(Alphanumeric) as char).collect()
}
