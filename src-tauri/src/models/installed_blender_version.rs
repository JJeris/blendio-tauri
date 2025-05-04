use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

#[derive(Default, Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct InstalledBlenderVersion {
    pub id: String,
    pub version: String,
    pub variant_type: String,
    pub download_url: Option<String>,
    pub is_default: bool,
    pub installation_directory_path: String,
    pub executable_file_path: String,
    //  NaiveDateTime,
    pub created: String,
    pub modified: String,
    pub accessed: String,
}
