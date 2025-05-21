use crate::models::BlenderRepoPath;
use sqlx::SqlitePool;

pub struct BlenderRepoPathRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> BlenderRepoPathRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, repo: &BlenderRepoPath) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO blender_repo_paths (id, repo_directory_path, is_default) VALUES (?, ?, ?)",
            repo.id,
            repo.repo_directory_path,
            repo.is_default
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch(
        &self,
        id: Option<&str>,
        limit: Option<i64>,
        repo_directory_path: Option<&str>,
    ) -> Result<Vec<BlenderRepoPath>, sqlx::Error> {
        if let Some(id) = id { // A (1.d.) if let Some()
            let item = sqlx::query_as::<_, BlenderRepoPath>(
                "SELECT * FROM blender_repo_paths WHERE id = ?",
            )
            .bind(id)
            .fetch_all(self.pool)
            .await?;
            Ok(item)
        } else if let Some(limit) = limit { // A (1.d.) if let Some()
            sqlx::query_as::<_, BlenderRepoPath>("SELECT * FROM blender_repo_paths LIMIT ?")
                .bind(limit)
                .fetch_all(self.pool)
                .await
        } else if let Some(repo_directory_path) = repo_directory_path { // A (1.d.) if let Some()
            let item = sqlx::query_as::<_, BlenderRepoPath>(
                "SELECT * FROM blender_repo_paths WHERE repo_directory_path = ?",
            )
            .bind(repo_directory_path)
            .fetch_all(self.pool)
            .await?;
            Ok(item)
        } else {
            sqlx::query_as::<_, BlenderRepoPath>("SELECT * FROM blender_repo_paths")
                .fetch_all(self.pool)
                .await
        }
    }

    pub async fn update(&self, repo: &BlenderRepoPath) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE blender_repo_paths SET repo_directory_path = ?, is_default = ?, modified = CURRENT_TIMESTAMP, accessed = CURRENT_TIMESTAMP WHERE id = ?",
            repo.repo_directory_path,
            repo.is_default,
            repo.id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM blender_repo_paths WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
