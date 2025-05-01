use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DownloadableBlenderVersion {
    pub url: String,
    pub app: String,
    pub version: String,
    pub risk_id: String,
    pub branch: String,
    pub patch: Option<String>,
    pub hash: String,
    pub platform: String,
    pub architecture: String,
    pub bitness: i32,
    pub file_mtime: i64,
    pub file_name: String,
    pub file_size: i64,
    pub file_extension: String,
    pub release_cycle: String,
    pub checksum: String,
}
