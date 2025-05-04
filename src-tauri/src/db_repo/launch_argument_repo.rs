use crate::models::LaunchArgument;
use sqlx::SqlitePool;

pub struct LaunchArgumentRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> LaunchArgumentRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, arg: &LaunchArgument) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO launch_arguments (id, is_default, argument_string, last_used_project_file_id, last_used_python_script_id) VALUES (?, ?, ?, ?, ?)",
            arg.id,
            arg.is_default,
            arg.argument_string,
            arg.last_used_project_file_id,
            arg.last_used_python_script_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch(
        &self,
        id: Option<&str>,
        limit: Option<i64>,
        argument_string: Option<&str>
    ) -> Result<Vec<LaunchArgument>, sqlx::Error> {
        if let Some(id) = id {
            let item =
                sqlx::query_as::<_, LaunchArgument>("SELECT * FROM launch_arguments WHERE id = ?")
                    .bind(id)
                    .fetch_all(self.pool)
                    .await?;
            Ok(item)
        } else if let Some(limit) = limit {
            sqlx::query_as::<_, LaunchArgument>("SELECT * FROM launch_arguments LIMIT ?")
                .bind(limit)
                .fetch_all(self.pool)
                .await
        } else if let Some(argument_string) = argument_string {
            let item = sqlx::query_as::<_, LaunchArgument>("SELECT * FROM launch_arguments WHERE argument_string = ?")
                .bind(argument_string)
                .fetch_all(self.pool)
                .await?;
            Ok(item)
        } else {
            sqlx::query_as::<_, LaunchArgument>("SELECT * FROM launch_arguments")
                .fetch_all(self.pool)
                .await
        }
    }

    pub async fn update(&self, arg: &LaunchArgument) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE launch_arguments SET is_default = ?, argument_string = ?, last_used_project_file_id = ?, last_used_python_script_id = ?, modified = CURRENT_TIMESTAMP, accessed = CURRENT_TIMESTAMP WHERE id = ?",
            arg.is_default,
            arg.argument_string,
            arg.last_used_project_file_id,
            arg.last_used_python_script_id,
            arg.id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM launch_arguments WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
