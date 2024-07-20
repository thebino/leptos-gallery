use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use axum::async_trait;
    use axum_session_auth::Authentication;
    use sqlx::SqlitePool;

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlAlbum {
        pub id: i64,
        pub albumcode: String,
        pub passcode: String,
    }

    use super::Album;

    impl Album {
        pub async fn get_album(id: i64, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlAlbum>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            Some(Album {
                id: sqluser.id,
                anonymous: false,
                username: sqluser.albumcode,
            })
        }

        pub async fn validate_credentials(
            albumcode: String,
            passcode: String,
            pool: &SqlitePool,
        ) -> Option<Self> {
            let sql_album =
                sqlx::query_as::<_, SqlAlbum>("SELECT * FROM users WHERE albumcode = ?")
                    .bind(albumcode.trim())
                    .fetch_one(pool)
                    .await
                    .ok()?;

            if sql_album.passcode.trim() == passcode.trim() {
                Some(Album {
                    id: sql_album.id,
                    anonymous: false,
                    username: sql_album.albumcode,
                })
            } else {
                None
            }
        }
    }

    #[async_trait]
    impl Authentication<Album, i64, SqlitePool> for Album {
        // This is run when the user has logged in and has not yet been Cached in the system.
        // Once ran it will load and cache the user.
        async fn load_user(id: i64, pool: Option<&SqlitePool>) -> anyhow::Result<Album> {
            let pool = pool.unwrap();

            Album::get_album(id, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Could not load user"))
        }

        // This function is used internally to determine if they are logged in or not.
        fn is_authenticated(&self) -> bool {
            !self.anonymous
        }

        fn is_active(&self) -> bool {
            !self.anonymous
        }

        fn is_anonymous(&self) -> bool {
            self.anonymous
        }
    }
}
