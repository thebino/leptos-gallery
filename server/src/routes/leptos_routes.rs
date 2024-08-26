use app::components::app::App;
use axum::{
    body::Body,
    extract::{Request, State},
    response::{IntoResponse, Response},
};
use leptos::provide_context;

use crate::LeptosAppState;

use super::server_functions::AuthSession;

/// Custom handler for leptos which provides additional context like `AppState` and `AuthSession`
pub async fn leptos_routes_handler(
    auth_session: AuthSession,
    State(app_state): State<LeptosAppState>,
    req: Request<Body>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        },
        App,
    );
    handler(req).await.into_response()
}
