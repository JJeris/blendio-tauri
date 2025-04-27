use sqlx::SqlitePool;

use crate::models::User;

pub struct UserRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_user(&self, name: &str, email: Option<&str>) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
            .bind(name)
            .bind(email)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn fetch_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
            .fetch_all(self.pool)
            .await?;
        Ok(users)
    }
}
