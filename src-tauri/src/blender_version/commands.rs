//! Contains the frontend exposed commands of the Blender version module.

use std::{fs::File, result};

use chrono::Utc;
use regex::Regex;
use tauri::{utils::platform::Target, AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;
use zip::ZipArchive;

use crate::{
    db_context::establish_connection, db_repo::{BlenderRepoPathRepository, InstalledBlenderVersionRepository, LaunchArgumentRepository, PythonScriptRepository}, file_system_utility, launch_argument, models::{BlenderRepoPath, DownloadableBlenderVersion, InstalledBlenderVersion}, AppState
};

/// Saglabāt instalētu Blender versiju datus
#[tauri::command]
pub async fn insert_installed_blender_version(
    state: tauri::State<'_, AppState>,
    executable_file_path: std::path::PathBuf
) -> Result<(), String> {
    let parent_dir = match executable_file_path.parent() {
        Some(val) => val,
        None => return Err(String::new()),
    };
    // TODO identify these values that are left as String::new(). Preferably identify this from the executable_file_path.
    let dir_name = match parent_dir.file_name() {
        Some(val) => val.to_string_lossy().to_string(),
        None => return Err(String::new()),
    };

    // Try to extract version and variant_type using regex
    let (version, variant_type) = {
        let re = match Regex::new(r"blender-(?P<version>\d+\.\d+(?:\.\d+)?)-(?P<variant>[^\-+]+)") {
            Ok(val) => val,
            Err(err) => return Err(String::new()),
        };
        if let Some(caps) = re.captures(&dir_name) {
            let version = caps.name("version").map(|m| m.as_str().to_string()).unwrap_or_default();
            let variant_type = caps.name("variant").map(|m| m.as_str().to_string()).unwrap_or_default();
            (version, variant_type)
        } else {
            (String::new(), String::new())
        }
    };
    
    let entry = InstalledBlenderVersion {
        id: Uuid::new_v4().to_string(),
        version: version,
        variant_type: variant_type,
        download_url: None,
        is_default: false,
        installation_directory_path: parent_dir.to_string_lossy().to_string(),
        executable_file_path: executable_file_path.to_string_lossy().to_string(),
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(), 
    };
    println!("{:?}", entry);
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
        println!("{:?}", repo_path);
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
            println!("{:?}", launcher_path);
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
    launch_arguments_id: Option<String>,
    python_script_id: Option<String>
) -> Result<(), String> {
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool);
    let launch_argument_repository = LaunchArgumentRepository::new(&state.pool);
    let python_script_repository = PythonScriptRepository::new(&state.pool);
    
    let mut installed_blender_version_list = match installed_blender_version_repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(_) => return Err(String::new()),
    };
    let instance = installed_blender_version_list.remove(0);
    match installed_blender_version_repository.update(&instance).await {
        Ok(_) => {},
        Err(err) => return Err(String::new()),
    }
    let mut final_launch_args: Vec<String> = vec![];
    match launch_arguments_id {
        Some(arg_id) => {
            let mut launch_argument_entry_list = match launch_argument_repository.fetch(Some(&arg_id), None, None).await {
                Ok(val) => val,
                Err(_) => return Err(String::new()),
            };
            if launch_argument_entry_list.is_empty() {
                return Err(String::new());
            }
            let entry = launch_argument_entry_list.remove(0);
            match launch_argument_repository.update(&entry).await {
                Ok(_) => {},
                Err(err) => return Err(String::new()),
            }
            let parsed_args: Vec<String> = entry.argument_string.split_whitespace().map(|s| s.to_string()).collect();
            final_launch_args.extend(parsed_args);
        }
        None => {}
    }
    match python_script_id {
        Some(script_id) => {
            let mut python_script_entry_list = match python_script_repository.fetch(Some(&script_id), None, None).await {
                Ok(val) => val,
                Err(_) => return Err(String::new()),
            };
            if python_script_entry_list.is_empty() {
                return Err(String::new());
            }
            let entry = python_script_entry_list.remove(0);
            match python_script_repository.update(&entry).await {
                Ok(_) => {},
                Err(err) => return Err(String::new()),
            }
            if !final_launch_args.contains(&"--python".to_string()) {
                final_launch_args.push("--python".to_string());
                final_launch_args.push(entry.script_file_path);
            } else if final_launch_args.contains(&"--python".to_string()) {
                final_launch_args.push(entry.script_file_path);
            }
        }
        None => {}
    }
    println!("{:?}, {:?}", instance.executable_file_path, final_launch_args);
    match file_system_utility::launch_executable(std::path::PathBuf::from(instance.executable_file_path), Some(final_launch_args)) {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
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
    is_default: bool,
) -> Result<(), String> {
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let mut results = match repository.fetch(Some(&id), None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if results.is_empty() {
        return Err(String::new())
    }
    let mut entry = results.remove(0);
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
            let new_default = entry.id == id;
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
    let blender_repo_path_repository = BlenderRepoPathRepository::new(&state.pool);
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut blender_repo_path_list = match blender_repo_path_repository.fetch(Some(&id), None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if blender_repo_path_list.is_empty() {
        return Err(String::new());
    }
    let blender_repo_path_entry = blender_repo_path_list.remove(0);
    let installed_blender_version_list = match installed_blender_version_repository.fetch(None, None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    println!("#################################");
    for version in installed_blender_version_list {
        println!("{:?}", version.installation_directory_path);
        println!("{:?}", blender_repo_path_entry.repo_directory_path);
        if version.installation_directory_path.starts_with(&blender_repo_path_entry.repo_directory_path) {
            match installed_blender_version_repository.delete(&version.id).await {
                Ok(_) => {},
                Err(err) => return Err(String::new()),
            }
        }
    } 
    match blender_repo_path_repository.delete(&id).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}
