use crate::AppState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use serde_json::json;
use tracing::error;

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, http::response::Response<String>> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) if auth_header == state.password.unwrap_or("".to_string()) => {
            Ok(next.run(req).await)
        }
        _ => {
            let response_body = json!(
                {
                    "error": "Unauthorized",
                    "message": "Authentication header missing or invalid"
                }
            );
            Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(http::header::CONTENT_TYPE, "application/json")
                .header(http::header::WWW_AUTHENTICATE, "Bearer")
                .body(serde_json::to_string(&response_body).unwrap())
                .map_err(|_| error!("Fail to build unauthorized response"))
                .unwrap())
        }
    }
}
