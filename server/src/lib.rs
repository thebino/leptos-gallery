use std::path::PathBuf;

use axum::extract::FromRef;
use leptos::LeptosOptions;
use leptos_router::RouteListing;
use sqlx::SqlitePool;

pub mod handlers;
pub mod middlewares;

#[derive(FromRef, Debug, Clone)]
pub struct LeptosAppState {
    pub leptos_options: LeptosOptions,
    pub pool: SqlitePool,
    pub root: PathBuf,
    pub routes: Vec<RouteListing>,
}

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub root: PathBuf,
    pub password: Option<String>
}
