use sqlx::migrate;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn init_db() -> SqlitePool {
    let database_url = "sqlite://photo.db";

    if !sqlx::Sqlite::database_exists(database_url)
        .await
        .unwrap_or(false)
    {
        sqlx::Sqlite::create_database(database_url).await.unwrap();
    }

    let pool = SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .unwrap();

    if let Err(e) = migrate!("../migrations").run(&pool).await {
        eprintln!("{e:?}");
    }

    pool
}
