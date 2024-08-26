#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::{self};
    use axum::response::Response;
    use axum::routing::{delete, post};
    use axum::{middleware, Router};
    use http::StatusCode;
    use http_body_util::BodyExt; // for `collect`
    use leptos::html::body;
    use serde_json::{json, Value};
    use server::handlers::album::add_album::add_album;
    use server::handlers::album::delete_album::delete_album;
    use server::middlewares::auth::auth_middleware;
    use server::AppState;
    use sqlx::{Row, SqlitePool};
    use std::fs;
    use std::path::PathBuf;
    use testdir::testdir;
    use tower::ServiceExt; // for `oneshot`

    #[sqlx::test]
    pub async fn delete_album_call_with_valid_credentials_should_succeed(
        pool: SqlitePool,
    ) -> anyhow::Result<()> {
        // given
        let dir: PathBuf = testdir!();
        sqlx::migrate!("../migrations").run(&pool).await?;

        let album_code = "ABCD1234";
        let path: &str = &*("/album/".to_string() + album_code);

        let state = AppState {
            pool: pool.clone(),
            root: dir.clone(),
            password: Some("secret".to_string()),
        };

        // add album to delete
        sqlx::query("INSERT INTO users (albumcode, passcode) VALUES ($1, $2)")
            .bind("ABCD1234".to_string())
            .bind("secret".to_string())
            .execute(&state.pool)
            .await
            .expect("Fail to insert new albumcode into database");

        // create album directory to delete
        let filepath = state.root.join(&album_code);
        fs::create_dir_all(filepath.clone()).expect("Fail to create album directory");

        let router = Router::new()
            .route("/album/:album_code", delete(delete_album))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
            .with_state(state.clone());

        // when
        let request = Request::builder()
            .method(hyper::Method::DELETE)
            .uri(path)
            .header(http::header::AUTHORIZATION, state.password.unwrap())
            .body(Body::empty())
            .unwrap();
        let response: Response<Body> = router.oneshot(request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        // album should be deleted from database
        dbg!(album_code.clone());
        let row = sqlx::query("SELECT COUNT(*) AS 'count!' FROM users WHERE albumcode = $1")
            .fetch_one(&pool)
            .await?;
        assert_eq!(row.get::<i32, _>("count!"), 0);

        // album should be deleted from filesystem
        assert!(!filepath.exists());

        Ok(())
    }
}
