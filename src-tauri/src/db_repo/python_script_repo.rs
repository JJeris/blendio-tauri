use crate::models::PythonScript;
use sqlx::SqlitePool;

pub struct PythonScriptRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> PythonScriptRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, script: &PythonScript) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO python_scripts (id, script_file_path) VALUES (?, ?) ON CONFLICT(script_file_path) DO NOTHING",
            script.id,
            script.script_file_path
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch(
        &self,
        id: Option<&str>,
        limit: Option<i64>,
        script_file_path: Option<&str>,
    ) -> Result<Vec<PythonScript>, sqlx::Error> {
        if let Some(id) = id { // A (1.d.) if let Some()
            let item =
                sqlx::query_as::<_, PythonScript>("SELECT * FROM python_scripts WHERE id = ?")
                    .bind(id)
                    .fetch_all(self.pool)
                    .await?;
            Ok(item)
        } else if let Some(limit) = limit { // A (1.d.) if let Some()
            sqlx::query_as::<_, PythonScript>("SELECT * FROM python_scripts LIMIT ?")
                .bind(limit)
                .fetch_all(self.pool)
                .await
        } else if let Some(script_file_path) = script_file_path { // A (1.d.) if let Some()
            let item = sqlx::query_as::<_, PythonScript>(
                "SELECT * FROM python_scripts WHERE script_file_path = ?",
            )
            .bind(script_file_path)
            .fetch_all(self.pool)
            .await?;
            Ok(item)
        } else {
            sqlx::query_as::<_, PythonScript>("SELECT * FROM python_scripts")
                .fetch_all(self.pool)
                .await
        }
    }

    pub async fn update(&self, script: &PythonScript) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE python_scripts SET script_file_path = ?, modified = CURRENT_TIMESTAMP, accessed = CURRENT_TIMESTAMP WHERE id = ?",
            script.script_file_path,
            script.id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM python_scripts WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
