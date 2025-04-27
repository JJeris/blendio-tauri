use sqlx::SqlitePool;

pub async fn establish_connection(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(database_url).await
}