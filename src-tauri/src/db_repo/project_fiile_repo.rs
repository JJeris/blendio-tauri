use crate::models::ProjectFile;
use sqlx::SqlitePool;

pub struct ProjectFileRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> ProjectFileRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, file: &ProjectFile) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO project_files (id, file_path, file_name, associated_series_json, last_used_blender_version_id) VALUES (?, ?, ?, ?, ?) ON CONFLICT(file_path) DO NOTHING",
            file.id,
            file.file_path,
            file.file_name,
            file.associated_series_json,
            file.last_used_blender_version_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch(
        &self,
        id: Option<&str>,
        limit: Option<i64>,
        file_path: Option<&str>,
    ) -> Result<Vec<ProjectFile>, sqlx::Error> {
        if let Some(id) = id {
            let item = sqlx::query_as::<_, ProjectFile>("SELECT * FROM project_files WHERE id = ?")
                .bind(id)
                .fetch_all(self.pool)
                .await?;
            Ok(item)
        } else if let Some(limit) = limit {
            sqlx::query_as::<_, ProjectFile>("SELECT * FROM project_files LIMIT ?")
                .bind(limit)
                .fetch_all(self.pool)
                .await
        } else if let Some(file_path) = file_path {
            let item =
                sqlx::query_as::<_, ProjectFile>("SELECT * FROM project_files WHERE file_path = ?")
                    .bind(file_path)
                    .fetch_all(self.pool)
                    .await?;
            Ok(item)
        } else {
            sqlx::query_as::<_, ProjectFile>("SELECT * FROM project_files")
                .fetch_all(self.pool)
                .await
        }
    }

    pub async fn update(&self, file: &ProjectFile) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE project_files SET file_path = ?, file_name = ?, associated_series_json = ?, last_used_blender_version_id = ?, modified = CURRENT_TIMESTAMP, accessed = CURRENT_TIMESTAMP WHERE id = ?",
            file.file_path,
            file.file_name,
            file.associated_series_json,
            file.last_used_blender_version_id,
            file.id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM project_files WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
