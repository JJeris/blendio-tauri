//! Contains the frontend exposed commands of the Blender version module.

use chrono::Utc;
use tauri::{utils::platform::Target, AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::{
    db_context::establish_connection,
    db_repo::{BlenderRepoPathRepository, UserRepository},
    models::{BlenderRepoPath, DownloadableBlenderVersion, User},
    AppState,
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
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let repo_directory_path = app.dialog().file().blocking_pick_folder();
    let entry = BlenderRepoPath {
        id: Uuid::new_v4().to_string(),
        repo_directory_path: match repo_directory_path {
            Some(val) => val.to_string(),
            None => return Err(String::new())
        }, 
        is_default: false,
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(),
    };
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let results = match repository.fetch(None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if (results.iter().any(|x| x.repo_directory_path == entry.repo_directory_path)) {
        return Ok(());
    }
    match repository.insert(&entry).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}

/// Atjaunināt Blender versiju lejupielādes/instalācijas lokāciju failu sistēmā.
#[tauri::command]
pub async fn update_blender_version_installation_location(
    state: tauri::State<'_, AppState>,
    id: String,
    repo_directory_path: std::path::PathBuf,
    is_default: bool,
) -> Result<(), String> {
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let mut entry = BlenderRepoPath {
        id: id.clone(),
        repo_directory_path: repo_directory_path.to_string_lossy().to_string(),
        is_default,
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(),
    };
    if is_default == true 
    {
        entry.is_default = false;
        match repository.update(&entry).await {
            Ok(_) => Ok(()),
            Err(err) => return Err(String::new()),
        }
    }
    else
    {
        let results = match repository.fetch(None, None).await {
            Ok(val) => val,
            Err(err) => return Err(String::new()),
        };
        for mut entry in results {
            let new_default = entry.id == id; // TODO fix.
            if entry.is_default != new_default
            {
                entry.is_default = new_default;
                match repository.update(&entry).await {
                    Ok(_) => {},
                    Err(err) => return Err(String::new()),
                }
            }
        }
        Ok(())
    }
}

/// Saņemt Blender versiju lejupielādes/instalācijas lokāciju failu sistēmā.
#[tauri::command]
pub async fn fetch_blender_version_installation_locations(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<BlenderRepoPath>, String> {
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let results = match repository.fetch(id.as_deref(), limit).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    Ok(results)
}

// Izdzēst Blender versiju lejupielādes/instalācijas lokāciju failu sistēmā.
#[tauri::command]
pub async fn delete_blender_version_installation_location(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let repository = BlenderRepoPathRepository::new(&state.pool);
    match repository.delete(&id).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}
