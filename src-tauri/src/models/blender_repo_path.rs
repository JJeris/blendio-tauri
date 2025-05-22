use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};

#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
pub struct BlenderRepoPath {
    pub id: String,
    pub repo_directory_path: String,
    pub is_default: bool,
    pub created: String,
    pub modified: String,
    pub accessed: String,
}
