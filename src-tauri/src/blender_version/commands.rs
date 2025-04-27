//! Contains the frontend exposed commands of the Blender version module.

use tauri::{AppHandle, Manager};

use crate::{db_context::establish_connection, db_repo::UserRepository, models::User};


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
pub async fn insert_downloadable_blender_version_data() -> Result<(), String> {
    Ok(())
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
    directory_path: std::path::PathBuf,
) -> Result<(), String> {
    Ok(())
}

pub fn get_database_url(app: &AppHandle) -> Result<String, String> {
    let base_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = base_dir.join("test.db");
    Ok(format!("sqlite://{}", db_path.to_string_lossy()))
}

#[tauri::command]
pub async fn insert_user(app: AppHandle, name: String, email: Option<String>) -> Result<(), String> {
    let db_url = get_database_url(&app)?; // (I show below where to put this helper)
    let pool = establish_connection(&db_url).await.map_err(|e| e.to_string())?;
    let repo = UserRepository::new(&pool);
    repo.insert_user(&name, email.as_deref()).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_users(app: AppHandle) -> Result<Vec<User>, String> {
    let db_url = get_database_url(&app)?;
    let pool = establish_connection(&db_url).await.map_err(|e| e.to_string())?;
    let repo = UserRepository::new(&pool);
    repo.fetch_all_users().await.map_err(|e| e.to_string())
}