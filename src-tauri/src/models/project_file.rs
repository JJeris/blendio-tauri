use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectFile {
    pub id: String,
    pub file_path: String,
    pub file_name: String,
    pub associated_series: String,
    pub last_used_blender_version_id: Option<String>,
    pub created: String,
    pub modified: String,
    pub accessed: String,
}
