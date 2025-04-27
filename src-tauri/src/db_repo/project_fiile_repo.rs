use sqlx::SqlitePool;
use crate::models::ProjectFile;

pub struct ProjectFileRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> ProjectFileRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, entry: &ProjectFile) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO project_files (id, file_path, file_name, associated_series, last_used_blender_version_id) VALUES (?, ?, ?, ?, ?)",
            entry.id,
            entry.file_path,
            entry.file_name,
            entry.associated_series,
            entry.last_used_blender_version_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch(&self, id: Option<&str>, limit: Option<i64>) -> Result<Vec<ProjectFile>, sqlx::Error> {
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
        } else {
            sqlx::query_as::<_, ProjectFile>("SELECT * FROM project_files")
                .fetch_all(self.pool)
                .await
        }
    }

    pub async fn update(&self, file: &ProjectFile) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE project_files SET file_path = ?, file_name = ?, associated_series = ?, last_used_blender_version_id = ?, modified = CURRENT_TIMESTAMP, accessed = CURRENT_TIMESTAMP WHERE id = ?",
            file.file_path,
            file.file_name,
            file.associated_series,
            file.last_used_blender_version_id,
            file.id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}