use sqlx::PgPool;

pub type DBPool = PgPool;

pub async fn connect_db() -> DBPool {
    let config = crate::Config::get();
    PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to the database")
}
