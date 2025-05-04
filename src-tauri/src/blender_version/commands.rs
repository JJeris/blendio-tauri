//! Contains the frontend exposed commands of the Blender version module.

use std::{fs::File, result};

use chrono::Utc;
use tauri::{utils::platform::Target, AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;
use zip::ZipArchive;

use crate::{
    db_context::establish_connection, db_repo::{BlenderRepoPathRepository, InstalledBlenderVersionRepository}, file_system_utility, models::{BlenderRepoPath, DownloadableBlenderVersion, InstalledBlenderVersion}, AppState
};

/// Saglabāt instalētu Blender versiju datus
#[tauri::command]
pub async fn insert_installed_blender_version(
    state: tauri::State<'_, AppState>,
    executable_file_path: std::path::PathBuf
) -> Result<(), String> {
    let parent_dir = match executable_file_path.parent() {
        Some(val) => val.to_string_lossy().to_string(),
        None => return Err(String::new()),
    };
    // TODO identify these values that are left as String::new(). Preferably identify this from the executable_file_path.
    // let dir_name = parent_dir.file_name()
    // .map(|n| n.to_string_lossy().to_string())
    // .unwrap_or_default();

    // // Try to extract version and variant_type using regex
    // let (version, variant_type) = {
    //     let re = Regex::new(r"blender-(?P<version>\d+\.\d+(?:\.\d+)?)(?:[-+](?P<variant>.*))?").unwrap();
    //     if let Some(caps) = re.captures(&dir_name) {
    //         let version = caps.name("version").map(|m| m.as_str().to_string()).unwrap_or_default();
    //         let variant_type = caps.name("variant").map(|m| m.as_str().to_string()).unwrap_or_default();
    //         (version, variant_type)
    //     } else {
    //         (String::new(), String::new())
    //     }
    // };
    
    let entry = InstalledBlenderVersion {
        id: Uuid::new_v4().to_string(),
        version: String::new(),
        variant_type: String::new(),
        download_url: None,
        is_default: false,
        installation_directory_path: parent_dir,
        executable_file_path: executable_file_path.to_string_lossy().to_string(),
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(), 
    };
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    match repository.insert(&entry).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}

/// Saglabāt instalētu Blender versiju datus
#[tauri::command]
pub async fn insert_and_refresh_installed_blender_versions(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let blender_repo_paths = match repository.fetch(None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };

    let installed_blender_version_repo = InstalledBlenderVersionRepository::new(&state.pool);

    for repo_path in blender_repo_paths {
        let directory_entries = match std::fs::read_dir(repo_path.repo_directory_path) {
            Ok(val) => val,
            Err(err) => return Err(String::new())
        };
        for entry in directory_entries {
            let entry = match entry {
                Ok(val) => val,
                Err(err) => return Err(String::new())
            };
            if !entry.path().is_dir() {
                continue;
            }
            let launcher_path = entry.path().join("blender-launcher.exe");
            if !launcher_path.exists() {
                continue;
            }
            let existing_entries = match installed_blender_version_repo.fetch(None, None, Some(&launcher_path.to_string_lossy().to_string())).await {
                Ok(val) => val,
                Err(err) => return Err(String::new())
            };
            if !existing_entries.is_empty() {
                continue;
            }
            match insert_installed_blender_version(state.clone(), launcher_path).await {
                Ok(_) => {},
                Err(err) => return Err(String::new())
            }
        }
        // TODO if directory, check if .../blender-launcher.exe exists.
        // TODO try add cfg target_os is this method would really differ from Windows to linux and macos (primarily Windows and Linux).
        // TODO if exists, call the insert_installed_blender_version method
        // TODO first check if such a Blender version is already registerd (check the paths). If so, then dont insert and dont update. Leave as is, and go to the next step.
    } 
    Ok(())
}

#[tauri::command]
pub async fn update_installed_blender_version(
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let entry_list = match repository.fetch(Some(id.as_str()), None, None).await { // todo id.as_deref(), None
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    let mut entry = entry_list[0].clone(); // TODO fix.
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
        let results = match repository.fetch(None, None, None).await {
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

#[tauri::command]
pub async fn fetch_installed_blender_versions(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    installed_blender_versions: Option<&str>
) -> Result<Vec<InstalledBlenderVersion>, String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let results = match repository.fetch(id.as_deref(), limit, installed_blender_versions.as_deref()).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    Ok(results)
}

// TODO update to use all platforms.
/// Izdēst Blender versiju
#[tauri::command]
pub async fn uninstall_and_delete_installed_blender_version_data(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let entry_list = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    let entry = entry_list[0].clone(); // TODO fix.
    file_system_utility::delete_directory(std::path::PathBuf::from(entry.installation_directory_path)).await?;
    match repository.delete(&entry.id).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}

// TODO update to use all platforms.
/// Iedarbināt Blender versijas instanci ar palaišanas argumentiem
#[tauri::command]
pub async fn launch_blender_version_with_launch_args(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    launch_args: Option<Vec<String>>,
    project_file_id: Option<std::path::PathBuf>,
) -> Result<(), String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let instance = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val[0].clone(),
        Err(_) => return Err(String::new()),
    };
    match repository.update(&instance).await {
        Ok(_) => {},
        Err(err) => return Err(String::new()),
    }
    file_system_utility::launch_executable(std::path::PathBuf::from(instance.executable_file_path), launch_args)?;
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

/// Lejupielādēt un instalēt Blender versiju
#[tauri::command]
pub async fn download_and_install_blender_version(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    archive_file_path: std::path::PathBuf,
    downloadable_blender_version: DownloadableBlenderVersion
) -> Result<(), String> {
    let mut entry = InstalledBlenderVersion {
        id:Uuid::new_v4().to_string(), 
        version: downloadable_blender_version.version, 
        variant_type: downloadable_blender_version.release_cycle, 
        download_url: Some(downloadable_blender_version.url),
        is_default: false, 
        installation_directory_path: String::new(), 
        executable_file_path: String::new(), 
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(),
    };
    let installation_directory_path = match file_system_utility::extract_archive(archive_file_path.clone()).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    entry.installation_directory_path = installation_directory_path.to_string_lossy().to_string();
    entry.executable_file_path = installation_directory_path.join("blender-launcher.exe").to_string_lossy().to_string();
    file_system_utility::delete_file(archive_file_path).await?;
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    match repository.insert(&entry).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
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
    repo_directory_path: std::path::PathBuf, // todo fix.
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
