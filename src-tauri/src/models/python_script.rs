use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PythonScript {
    pub id: String,
    pub script_file_path: String,
    pub created: String,
    pub modified: String,
    pub accessed: String,
}
