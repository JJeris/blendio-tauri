use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LaunchArgument {
    pub id: String,
    pub is_default: bool,
    pub argument_string: String,
    pub last_used_project_file_id: Option<String>,
    pub last_used_python_script_id: Option<String>,
    pub created: String,
    pub modified: String,
    pub accessed: String,
}
