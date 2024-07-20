use std::path::PathBuf;
use app::components::app::App;
use app::models::user::Album;
use axum::routing::{delete, get};
use axum::routing::post;
use axum::{middleware, Router};
use axum_session::SessionConfig;
use axum_session::SessionLayer;
use axum_session::SessionSqlitePool;
use axum_session::SessionStore;
use axum_session_auth::AuthConfig;
use axum_session_auth::AuthSessionLayer;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tracing_subscriber::fmt;

use crate::db::init_db;

pub mod auth;
pub(crate) mod db;
pub(crate) mod fileserv;
pub mod handlers;

use crate::handlers::album::add_album::add_album;
use crate::handlers::cart::add_to_cart;
use crate::handlers::cart::get_cart;
use crate::handlers::leptos_routes::leptos_routes_handler;
use crate::handlers::server_functions::server_fn_handler;

use server::LeptosAppState;
use server::AppState;
use sqlx::SqlitePool;
use server::handlers::album::delete_album::delete_album;
use server::middlewares::auth::auth_middleware;
use crate::handlers::album::add_item::add_item;
use crate::handlers::album::delete_item::delete_item;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .with_target(false)
            .finish(),
    )
    .expect("Unable to set global tracing subscriber");

    let pool = init_db().await;

    let session_config = SessionConfig::default().with_table_name("axum_sessions");
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(None);
    let session_store = SessionStore::<SessionSqlitePool>::new(
        Some(SessionSqlitePool::from(pool.clone())),
        session_config,
    )
    .await
    .unwrap();

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let root_path = PathBuf::from("../public/");

    let leptos_app_state = LeptosAppState {
        leptos_options,
        pool: pool.clone(),
        root: root_path.clone(),
        routes: routes.clone(),
    };

    let app_state = AppState {
        pool: pool.clone(),
        root: root_path,
        password: Some("secret".to_string()) // TODO: read secret from config
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/album", post(add_album))
        .route("/api/album/:album_code", delete(delete_album))
        .route("/api/album/:album_code", post(add_item))
        .route("/api/album/:album_code/:item_id", delete(delete_item))
        .with_state(app_state.clone())
        .route_layer(middleware::from_fn_with_state(app_state, auth_middleware))
        .route("/get_cart", post(get_cart))
        .route("/add_to_cart", post(add_to_cart))
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(
            AuthSessionLayer::<Album, i64, SessionSqlitePool, SqlitePool>::new(Some(pool))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(leptos_app_state);

    // run our app with hyper
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // `axum::Server` is a re-export of `hyper::Server`
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
