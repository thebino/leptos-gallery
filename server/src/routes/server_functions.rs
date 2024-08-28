use app::models::user::Album;
use axum::{
    body::Body,
    extract::{Path, Request, State},
    response::IntoResponse,
};
use axum_session::SessionSqlitePool;
use leptos::provide_context;
use leptos_axum::handle_server_fns_with_context;
use log::info;
use sqlx::SqlitePool;

use crate::LeptosAppState;
#[allow(dead_code)]
pub type AuthSession = axum_session_auth::AuthSession<Album, i64, SessionSqlitePool, SqlitePool>;

/// Handler for server functions advanced with `AuthSession` and `AppState`
#[allow(dead_code)]
pub async fn server_fn_handler(
    State(app_state): State<LeptosAppState>,
    auth_session: AuthSession,
    path: Path<String>,
    request: Request<Body>,
) -> impl IntoResponse {
    info!("{:?}", path);

    handle_server_fns_with_context(
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        },
        request,
    )
    .await
}

