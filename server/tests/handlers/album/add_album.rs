#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::{self};
    use axum::response::Response;
    use axum::routing::post;
    use axum::{middleware, Router};
    use http::StatusCode;
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use server::handlers::album::add_album::add_album;
    use server::handlers::album::Album;
    use server::middlewares::auth::auth_middleware;
    use server::AppState;
    use sqlx::SqlitePool;
    use std::path::PathBuf;
    use testdir::testdir;
    use tower::ServiceExt; // for `oneshot`

    #[sqlx::test]
    pub async fn create_album_call_without_auth_header_should_fail(
        pool: SqlitePool,
    ) -> anyhow::Result<()> {
        // given
        let dir: PathBuf = testdir!();
        sqlx::migrate!("../migrations").run(&pool).await?;
        let path = "/";
        let app_state = AppState {
            pool,
            root: dir,
            password: Some("secret".to_string()),
        };
        let router = Router::new()
            .route(path, post(add_album))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            ))
            .with_state(app_state);

        // when
        let request = Request::builder()
            .method(hyper::Method::POST)
            .uri(path)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&Album {
                    id: "a586c2b1-cf3d-4804-8fd7-d25cc21da51e".to_string(),
                })
                .unwrap(),
            )
            .unwrap();

        let response: Response<Body> = router.oneshot(request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        Ok(())
    }

    #[sqlx::test]
    pub async fn create_album_call_with_invalid_credentials_should_fail(
        pool: SqlitePool,
    ) -> anyhow::Result<()> {
        // given
        let dir: PathBuf = testdir!();
        sqlx::migrate!("../migrations").run(&pool).await?;
        let path = "/";
        let app_state = AppState {
            pool,
            root: dir,
            password: Some("secret".to_string()),
        };
        let router = Router::new()
            .route(path, post(add_album))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            ))
            .with_state(app_state);

        // when
        let request = Request::builder()
            .method(hyper::Method::POST)
            .uri(path)
            .header(http::header::AUTHORIZATION, "invalid")
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&Album {
                    id: "a586c2b1-cf3d-4804-8fd7-d25cc21da51e".to_string(),
                })
                .unwrap(),
            )
            .unwrap();

        let response: Response<Body> = router.oneshot(request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response
                .headers()
                .get(http::header::WWW_AUTHENTICATE)
                .unwrap(),
            "Bearer"
        );

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        let expected_body = json!(
            {
                "error": "Unauthorized",
                "message": "Authentication header missing or invalid"
            }
        );
        let expected_value = serde_json::to_value(expected_body).unwrap();
        assert_eq!(body, expected_value);

        Ok(())
    }

    #[sqlx::test]
    pub async fn create_album_call_with_valid_credentials_should_succeed(
        pool: SqlitePool,
    ) -> anyhow::Result<()> {
        // given
        let dir: PathBuf = testdir!();
        sqlx::migrate!("../migrations").run(&pool).await?;
        let path = "/";
        let state = AppState {
            pool: pool.clone(),
            root: dir.clone(),
            password: Some("secret".to_string()),
        };
        let router = Router::new()
            .route(path, post(add_album))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
            .with_state(state.clone());

        // when
        let request = Request::builder()
            .method(hyper::Method::POST)
            .uri(path)
            .header(http::header::AUTHORIZATION, state.password.unwrap())
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body("new_passcode".to_string())
            .unwrap();
        let response: Response<Body> = router.oneshot(request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::CREATED);
        assert!(response.headers().get(http::header::LOCATION).is_some());
        let album_code = response
            .headers()
            .get(http::header::LOCATION)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let rows = sqlx::query("SELECT * FROM users WHERE albumcode = $1")
            .bind(album_code.clone())
            .fetch_all(&pool)
            .await?;
        assert_eq!(rows.iter().count(), 1);

        let filepath = dir.join(album_code);
        assert!(filepath.exists());

        Ok(())
    }
}
