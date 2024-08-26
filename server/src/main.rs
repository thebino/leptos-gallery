use app::components::app::App;
use app::models::user::Album;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use axum_session::SessionConfig;
use axum_session::SessionLayer;
use axum_session::SessionSqlitePool;
use axum_session::SessionStore;
use axum_session_auth::AuthConfig;
use axum_session_auth::AuthSessionLayer;
use leptos::*;
use leptos_axum::generate_route_list;
use leptos_axum::LeptosRoutes;
use routes::album::add_album::add_album;
use routes::album::add_item::add_item;
use routes::album::delete_album::delete_album;
use routes::album::delete_item::delete_item;
use routes::album::get_album::get_album;
use routes::album::get_item::get_item;
use routes::fileserv::file_and_error_handler;
use std::path::PathBuf;
use tracing_subscriber::fmt;

use crate::db::init_db;

pub(crate) mod db;
pub(crate) mod middleware;
pub(crate) mod routes;

pub mod config;

use crate::routes::leptos_routes::leptos_routes_handler;
use crate::routes::server_functions::server_fn_handler;

use server::config::Config;
use server::middleware::auth::auth_middleware;
use server::AppState;
use server::LeptosAppState;
use sqlx::SqlitePool;

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

    let config = Config::new("config.toml");
    let root_path = PathBuf::from(config.root_dir);

    let leptos_app_state = LeptosAppState {
        leptos_options,
        pool: pool.clone(),
        root: root_path.clone(),
        routes: routes.clone(),
    };

    let app_state = AppState {
        pool: pool.clone(),
        root: root_path,
        password: config.secret,
    };

    // build our application with a route
    let app: Router = Router::new()
        .route("/api/album", post(add_album))
        .route("/api/album/:album_code", get(get_album))
        .route("/api/album/:album_code", delete(delete_album))
        .route("/api/album/:album_code", post(add_item))
        .route("/api/album/:album_code/:item_id", delete(delete_item))
        .route("/api/album/:album_code/:item_id", get(get_item))
        .with_state(app_state.clone())
        .route_layer(axum::middleware::from_fn_with_state(
            app_state,
            auth_middleware,
        ))
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
