use crate::models::InstalledBlenderVersion;
use sqlx::SqlitePool;

pub struct InstalledBlenderVersionRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> InstalledBlenderVersionRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, entry: &InstalledBlenderVersion) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO installed_blender_versions (id, version, variant_type, download_url, is_default, installation_directory_path, executable_file_path) VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT(executable_file_path) DO NOTHING",
            entry.id,
            entry.version,
            entry.variant_type,
            entry.download_url,
            entry.is_default,
            entry.installation_directory_path,
            entry.executable_file_path
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch(
        &self,
        id: Option<&str>,
        limit: Option<i64>,
        executable_file_path: Option<&str>,
    ) -> Result<Vec<InstalledBlenderVersion>, sqlx::Error> {
        if let Some(id) = id {
            // A (1.d.) if let Some()
            let item = sqlx::query_as::<_, InstalledBlenderVersion>(
                "SELECT * FROM installed_blender_versions WHERE id = ?",
            )
            .bind(id)
            .fetch_all(self.pool)
            .await?;
            Ok(item)
        } else if let Some(limit) = limit {
            // A (1.d.) if let Some()
            sqlx::query_as::<_, InstalledBlenderVersion>(
                "SELECT * FROM installed_blender_versions LIMIT ?",
            )
            .bind(limit)
            .fetch_all(self.pool)
            .await
        } else if let Some(executable_file_path) = executable_file_path {
            // A (1.d.) if let Some()
            let item = sqlx::query_as::<_, InstalledBlenderVersion>(
                "SELECT * FROM installed_blender_versions WHERE executable_file_path = ?",
            )
            .bind(executable_file_path)
            .fetch_all(self.pool)
            .await?;
            Ok(item)
        } else {
            sqlx::query_as::<_, InstalledBlenderVersion>("SELECT * FROM installed_blender_versions")
                .fetch_all(self.pool)
                .await
        }
    }

    pub async fn update(&self, version: &InstalledBlenderVersion) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE installed_blender_versions SET version = ?, variant_type = ?, download_url = ?, is_default = ?, installation_directory_path = ?, executable_file_path = ?, modified = CURRENT_TIMESTAMP, accessed = CURRENT_TIMESTAMP WHERE id = ?",
            version.version,
            version.variant_type,
            version.download_url,
            version.is_default,
            version.installation_directory_path,
            version.executable_file_path,
            version.id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM installed_blender_versions WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
