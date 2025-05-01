//! Contains the frontend exposed commands of the Blender version module.

use chrono::Utc;
use tauri::{utils::platform::Target, AppHandle, Manager};
use uuid::Uuid;

use crate::{
    db_context::establish_connection,
    db_repo::{BlenderRepoPathRepository, UserRepository},
    models::{BlenderRepoPath, DownloadableBlenderVersion, User},
};

/// Saglabāt instalētu Blender versiju datus
#[tauri::command]
pub async fn insert_installed_blender_version_data() -> Result<(), String> {
    Ok(())
}

/// Iedarbināt Blender versijas instanci ar palaišanas argumentiem
#[tauri::command]
pub async fn run_blender_version(
    launch_args: Option<Vec<String>>,
    blend_file_path: Option<std::path::PathBuf>,
) -> Result<(), String> {
    // You can handle the options like this:
    if let Some(args) = &launch_args {
        println!("Launch arguments: {:?}", args);
    } else {
        println!("No launch arguments provided.");
    }
    if let Some(path) = &blend_file_path {
        println!("Blend file path: {:?}", path);
    } else {
        println!("No blend file path provided.");
    }
    Ok(())
}

/// Saglabāt lejupielādējamu Blender versiju datus
#[tauri::command]
pub async fn get_downloadable_blender_version_data(
) -> Result<Vec<DownloadableBlenderVersion>, String> {
    let response =
        match reqwest::get("https://builder.blender.org/download/daily/?format=json&v=2").await {
            Ok(val) => val,
            Err(err) => return Err(format!("{}", err)),
        };
    let response_json: Vec<DownloadableBlenderVersion> = match response.json().await {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };
    #[cfg(target_os = "windows")]
    let filtered_data = response_json
        .into_iter()
        .filter(|p| {
            p.bitness == 64
                && p.platform == "windows"
                && p.architecture == "amd64"
                && p.file_extension == "zip"
        })
        .collect();

    #[cfg(target_os = "macos")]
    let filtered_data = response_json
        .into_iter()
        .filter(|p| {
            p.bitness == 64
                && p.platform == "darwin"
                && p.architecture == "arm64"
                && p.file_extension == "dmg"
        })
        .collect();

    #[cfg(target_os = "linux")]
    let filtered_data = response_json
        .into_iter()
        .filter(|p| {
            p.bitness == 64
                && p.platform == "linux"
                && p.architecture == "x86_64"
                && p.file_extension == "xz"
        })
        .collect();
    return Ok(filtered_data);
}

/// Lejupielādēt Blender versiju
#[tauri::command]
pub async fn download_and_install_blender_version(download_url: String) -> Result<(), String> {
    Ok(())
}

/// Izdēst Blender versiju
#[tauri::command]
pub async fn uninstall_blender_version(
    blender_directory_path: std::path::PathBuf,
) -> Result<(), String> {
    Ok(())
}

/// Saglabāt Blender versiju lejupielādes/instalācijas lokāciju failu sistēmā.
#[tauri::command]
pub async fn insert_blender_version_installation_location(
    app: AppHandle,
    directory_path: std::path::PathBuf,
) -> Result<(), String> {
    let db_url = get_database_url(&app)?;
    let pool = sqlx::SqlitePool::connect(&db_url)
        .await
        .map_err(|e| e.to_string())?;

    let repo = BlenderRepoPath {
        id: Uuid::new_v4().to_string(),
        repo_directory_path: directory_path.to_string_lossy().into_owned(),
        is_default: false,
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(),
    };

    let repository = BlenderRepoPathRepository::new(&pool);
    repository.insert(&repo).await.map_err(|e| e.to_string())
}

/// Saglabāt Blender versiju lejupielādes/instalācijas lokāciju failu sistēmā.
#[tauri::command]
pub async fn update_blender_version_installation_location(
    repo_directory_path: std::path::PathBuf
) -> Result<(), String> {
    Ok(())
}

pub fn get_database_url(app: &AppHandle) -> Result<String, String> {
    let base_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = base_dir.join("test.db");
    Ok(format!("sqlite://{}", db_path.to_string_lossy()))
}

// #[tauri::command]
// pub async fn insert_user(
//     app: AppHandle,
//     name: String,
//     email: Option<String>,
// ) -> Result<(), String> {
//     let db_url = get_database_url(&app)?; // (I show below where to put this helper)
//     let pool = establish_connection(&db_url)
//         .await
//         .map_err(|e| e.to_string())?;
//     let repo = UserRepository::new(&pool);
//     repo.insert_user(&name, email.as_deref())
//         .await
//         .map_err(|e| e.to_string())
// }

// #[tauri::command]
// pub async fn get_users(app: AppHandle) -> Result<Vec<User>, String> {
//     let db_url = get_database_url(&app)?;
//     let pool = establish_connection(&db_url)
//         .await
//         .map_err(|e| e.to_string())?;
//     let repo = UserRepository::new(&pool);
//     repo.fetch_all_users().await.map_err(|e| e.to_string())
// }
