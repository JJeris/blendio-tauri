use chrono::Utc;
use regex::Regex;
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use crate::{
    db_repo::{BlenderRepoPathRepository, InstalledBlenderVersionRepository, LaunchArgumentRepository, PythonScriptRepository}, file_system_utility::{self, show_ok_notification}, launch_argument, models::{BlenderRepoPath, DownloadableBlenderVersion, InstalledBlenderVersion}, AppState
};

/// ID: BV_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn insert_installed_blender_version(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    executable_file_path: std::path::PathBuf
) -> Result<(), String> {
    let parent_dir = match executable_file_path.parent() {
        Some(val) => val,
        None => {
            show_ok_notification(app.clone(), format!("Failed to get file path parent"), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to get file path parent"))
        },
    };
    let dir_name = match parent_dir.file_name() {
        Some(val) => val.to_string_lossy().to_string(),
        None => {
            show_ok_notification(app.clone(), format!("Failed to get file name"), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to get file name"))
        },
    };
    let re = match Regex::new(r"blender-(?P<version>\d+\.\d+(?:\.\d+)?)-(?P<variant>[^\-+]+)") {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to construct regex: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to construct regex: {:?}", err))
        },
    };
    let mut version = String::new();
    let mut variant_type = String::new();
    if let Some(caps) = re.captures(&dir_name) {
        version = caps.name("version").map(|m| m.as_str().to_string()).unwrap_or_default();
        variant_type = caps.name("variant").map(|m| m.as_str().to_string()).unwrap_or_default();
    }
    // Try to extract version and variant_type using regex
    let entry = InstalledBlenderVersion {
        id: uuid::Uuid::new_v4().to_string(),
        version: version,
        variant_type: variant_type,
        download_url: None,
        is_default: false,
        installation_directory_path: parent_dir.to_string_lossy().to_string(),
        executable_file_path: executable_file_path.to_string_lossy().to_string(),
        created: chrono::Utc::now().to_rfc3339(),
        modified: chrono::Utc::now().to_rfc3339(),
        accessed: chrono::Utc::now().to_rfc3339(), 
    };
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    match repository.insert(&entry).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to insert installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert installed Blender version: {:?}", err))
        },
    }
}

/// ID: BV_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn insert_and_refresh_installed_blender_versions(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let blender_repo_paths_repo = BlenderRepoPathRepository::new(&state.pool);
    let blender_repo_paths = match blender_repo_paths_repo.fetch(None, None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch Blender repo paths: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err))
        },
    };
    let installed_blender_version_repo = InstalledBlenderVersionRepository::new(&state.pool);
    for repo_path in blender_repo_paths {
        let directory_entries = match std::fs::read_dir(repo_path.repo_directory_path) {
            Ok(val) => val,
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to insert and refresh installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to insert and refresh installed Blender version: {:?}", err))
            },
        };
        for entry in directory_entries {
            let entry = match entry {
                Ok(val) => val,
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to insert and refresh installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to insert and refresh installed Blender version: {:?}", err))
                },
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
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
                },
            };
            if !existing_entries.is_empty() {
                continue;
            }
            match insert_installed_blender_version(app.clone(), state.clone(), launcher_path).await {
                Ok(_) => {},
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to insert and refresh installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to insert and refresh installed Blender version: {:?}", err))
                },
            }
        }
    }
    let current_entries = match installed_blender_version_repo.fetch(None, None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err));
        },
    };
    for entry in current_entries {
        let path = std::path::Path::new(&entry.executable_file_path);
        if !path.exists() {
            match installed_blender_version_repo.delete(&entry.id).await {
                Ok(_) => {},
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to delete Blender version entry: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to delete Blender version entry: {:?}", err));
                },
            }
        }
    }
    Ok(())
}

/// ID: BV_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn update_installed_blender_version(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut results = match repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
        },
    };
    if results.is_empty() {
        show_ok_notification(app.clone(), format!("Failed to fetch installed Blender version by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
        return Err(format!("Failed to fetch installed Blender version by ID"))
    }
    let mut entry = results.remove(0);
    if is_default == true
    {
        entry.is_default = false;
        match repository.update(&entry).await {
            Ok(_) => Ok(()),
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to update existing installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to update existing installed Blender version: {:?}", err))
            },
        }
    }
    else
    {
        let results = match repository.fetch(None, None, None).await {
            Ok(val) => val,
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
            },
        };
        for mut entry in results {
            let new_default = entry.id == id; // TODO fix.
            if entry.is_default != new_default
            {
                entry.is_default = new_default;
                match repository.update(&entry).await {
                    Ok(_) => {},
                    Err(err) => {
                        show_ok_notification(app.clone(), format!("Failed to update installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                        return Err(format!("Failed to update installed Blender versions: {:?}", err))
                    },
                }
            }
        }
        Ok(())
    }
}

/// ID: BV_004
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn fetch_installed_blender_versions(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    executable_file_path: Option<&str>
) -> Result<Vec<InstalledBlenderVersion>, String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut results = match repository.fetch(id.as_deref(), limit, executable_file_path.as_deref()).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
        },
    };
    // Sort DESC
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed));
    Ok(results)
}
/// ID: BV_005
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn uninstall_and_delete_installed_blender_version_data(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let confirmation= file_system_utility::show_ask_notification(app.clone(), format!("Are you sure you want to delete this installed Blender version?"), tauri_plugin_dialog::MessageDialogKind::Warning);
    if confirmation == false {
        return Ok(());
    }
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut installed_blender_version_list = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
        },
    };
    if installed_blender_version_list.is_empty() {
        show_ok_notification(app.clone(), format!("Failed to fetch installed Blender version by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
        return Err(format!("Failed to fetch installed Blender version by ID"))
    }
    let entry = installed_blender_version_list.remove(0);
    match file_system_utility::delete_directory(std::path::PathBuf::from(entry.installation_directory_path)).await {
        Ok(_) => {},
        Err(err) => {
            match repository.delete(&entry.id).await {
                Ok(_) => {},
                Err(err) => show_ok_notification(app.clone(), format!("Failed to delete installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error),
            }
            show_ok_notification(app.clone(), format!("Failed to delete installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to delete installed Blender versions: {:?}", err))
        },
    }
    match repository.delete(&entry.id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to delete installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender version: {:?}", err))
        },
    }
}

/// ID: BV_006
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn launch_blender_version_with_launch_args(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    launch_arguments_id: Option<String>,
    python_script_id: Option<String>
) -> Result<(), String> {
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool);
    let launch_argument_repository = LaunchArgumentRepository::new(&state.pool);
    let python_script_repository = PythonScriptRepository::new(&state.pool);
    let mut installed_blender_version_list = match installed_blender_version_repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
        },
    };
    if installed_blender_version_list.is_empty() {
        show_ok_notification(app.clone(), format!("Failed to fetch installed Blender version by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
        return Err(format!("Failed to fetch installed Blender version by ID"))
    }
    let instance = installed_blender_version_list.remove(0);
    match installed_blender_version_repository.update(&instance).await {
        Ok(_) => {},
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to update installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to update installed Blender version: {:?}", err))
        },
    }
    let mut final_launch_args: Vec<String> = vec![];
    match launch_arguments_id {
        Some(arg_id) => {
            let mut launch_argument_entry_list = match launch_argument_repository.fetch(Some(&arg_id), None, None).await {
                Ok(val) => val,
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to fetch launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to fetch launch arguments: {:?}", err))
                },
            };
            if launch_argument_entry_list.is_empty() {
                show_ok_notification(app.clone(), format!("Failed to fetch launch argument by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to fetch launch argument by ID"))
            }
            let entry = launch_argument_entry_list.remove(0);
            match launch_argument_repository.update(&entry).await {
                Ok(_) => {},
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to update launch argument: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to update launch argument: {:?}", err))
                },
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
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to fetch python scripts: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to fetch python scripts: {:?}", err))
                },
            };
            if python_script_entry_list.is_empty() {
                show_ok_notification(app.clone(), format!("Failed to fetch python script by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to fetch python script by ID"))
            }
            let entry = python_script_entry_list.remove(0);
            match python_script_repository.update(&entry).await {
                Ok(_) => {},
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to update python script: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to update python script: {:?}", err))
                },
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
    match file_system_utility::launch_executable(std::path::PathBuf::from(instance.executable_file_path), Some(final_launch_args)) {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to launch installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to launch installed Blender version: {:?}", err))
        },
    }
}

/// ID: BV_007
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn get_downloadable_blender_version_data(
    app: AppHandle,
) -> Result<Vec<DownloadableBlenderVersion>, String> {
    let response =
        match reqwest::get("https://builder.blender.org/download/daily/?format=json&v=2").await {
            Ok(val) => val,
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to fetch downloadable Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to fetch downloadable Blender versions: {:?}", err))
            },
        };
    let response_json: Vec<DownloadableBlenderVersion> = match response.json().await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch downloadable Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch downloadable Blender versions: {:?}", err))
        },
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

/// ID: BV_008
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn download_and_install_blender_version(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    archive_file_path: std::path::PathBuf,
    downloadable_blender_version: DownloadableBlenderVersion
) -> Result<(), String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut entry = InstalledBlenderVersion {
        id:Uuid::new_v4().to_string(), 
        version: downloadable_blender_version.version, 
        variant_type: downloadable_blender_version.release_cycle, 
        download_url: Some(downloadable_blender_version.url),
        is_default: false, 
        installation_directory_path: String::new(), 
        executable_file_path: String::new(), 
        created: chrono::Utc::now().to_rfc3339(),
        modified: chrono::Utc::now().to_rfc3339(),
        accessed: chrono::Utc::now().to_rfc3339(),
    };
    let installation_directory_path = match file_system_utility::extract_archive(archive_file_path.clone()).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to extract downloaded Blender versions files from archive file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to extract downloaded Blender versions files from archive file: {:?}", err))
        },
    };
    entry.installation_directory_path = installation_directory_path.to_string_lossy().to_string();
    entry.executable_file_path = installation_directory_path.join("blender-launcher.exe").to_string_lossy().to_string();
    match file_system_utility::delete_file(archive_file_path).await {
        Ok(_) => {},
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to delete downloaded archive file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to delete downloaded archive file: {:?}", err))
        },
    }
    let mut existing_entries = match repository.fetch(None, None, Some(&entry.executable_file_path)).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch Blender repo paths: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err))
        },
    };
    if existing_entries.is_empty() {
        match repository.insert(&entry).await {
            Ok(_) => Ok(()),
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to insert installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to insert installed Blender version: {:?}", err));
            },
        }
    }
    else
    {
        let mut old_entry = existing_entries.remove(0);
        old_entry.version = entry.version; 
        old_entry.variant_type = entry.variant_type; 
        old_entry.download_url = entry.download_url;
        old_entry.is_default = entry.is_default;
        old_entry.installation_directory_path = entry.installation_directory_path;
        old_entry.executable_file_path = entry.executable_file_path;
        old_entry.created = chrono::Utc::now().to_rfc3339();
        old_entry.modified = chrono::Utc::now().to_rfc3339();
        old_entry.accessed = chrono::Utc::now().to_rfc3339();
        match repository.update(&old_entry).await {
            Ok(_) => Ok(()),
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to insert installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to insert installed Blender version: {:?}", err))
            }
        }
    }
}


/// ID: BV_009
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn insert_blender_version_installation_location(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let repo_directory_path_option = match file_system_utility::get_directory_from_file_explorer(app.clone(), state.clone()).await {
        Ok(val) => val,
        Err(_) => return Ok(()),
    };
    let repo_directory_path = match repo_directory_path_option {
        Some(val) => val,
        None => return Ok(()),
    };
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let results = match repository.fetch(None, None, repo_directory_path.to_str()).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch Blender repo path: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch Blender repo path: {:?}", err))
        },
    };
    if !results.is_empty() {
        return Ok(());
    }
    let entry = BlenderRepoPath {
        id: uuid::Uuid::new_v4().to_string(),
        repo_directory_path: repo_directory_path.to_string_lossy().to_string(), 
        is_default: false,
        created: chrono::Utc::now().to_rfc3339(),
        modified: chrono::Utc::now().to_rfc3339(),
        accessed: chrono::Utc::now().to_rfc3339(),
    };
    match repository.insert(&entry).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to insert Blender repo path: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert Blender repo path: {:?}", err))
        },
    }
}

/// ID: BV_010
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn update_blender_version_installation_location(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let mut results = match repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch Blender repo paths: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err))
        },
    };
    if results.is_empty() {
        show_ok_notification(app.clone(), format!("Failed to fetch Blender repo path by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
        return Err(format!("Failed to fetch Blender repo path by ID"))
    }
    let mut entry = results.remove(0);
    if is_default == true 
    {
        entry.is_default = false;
        match repository.update(&entry).await {
            Ok(_) => Ok(()),
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to update Blender repo path: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to update Blender repo path: {:?}", err))
            },
        }
    }
    else
    {
        let results = match repository.fetch(None, None, None).await {
            Ok(val) => val,
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to fetch Blender repo paths: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to fetch Blender repo paths: {:?}", err))
            },
        };
        for mut entry in results {
            let new_default = entry.id == id;
            if entry.is_default != new_default
            {
                entry.is_default = new_default;
                match repository.update(&entry).await {
                    Ok(_) => {},
                    Err(err) => {
                        show_ok_notification(app.clone(), format!("Failed to update Blender repo paths: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                        return Err(format!("Failed to update Blender repo paths: {:?}", err))
                    },
                }
            }
        }
        Ok(())
    }
}

/// ID: BV_011
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn fetch_blender_version_installation_locations(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<&str>,
    limit: Option<i64>,
    repo_directory_path: Option<&str>
) -> Result<Vec<BlenderRepoPath>, String> {
    let repository = BlenderRepoPathRepository::new(&state.pool);
    let results = match repository.fetch(id.as_deref(), limit, repo_directory_path).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch Blender repo paths: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err))
        },
    };
    Ok(results)
}

/// ID: BV_012
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn delete_blender_version_installation_location(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let confirmation= file_system_utility::show_ask_notification(app.clone(), format!("Are you sure you want to delete this Blender installation location?"), tauri_plugin_dialog::MessageDialogKind::Warning);
    if confirmation == false {
        return Ok(());
    }
    let blender_repo_path_repository = BlenderRepoPathRepository::new(&state.pool);
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut blender_repo_path_list = match blender_repo_path_repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch Blender repo paths: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err))
        },
    };
    if blender_repo_path_list.is_empty() {
            show_ok_notification(app.clone(), format!("Failed to fetch Blender repo paths by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch Blender repo paths by ID"))
    }
    let blender_repo_path_entry = blender_repo_path_list.remove(0);
    let installed_blender_version_list = match installed_blender_version_repository.fetch(None, None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
        },
    };
    for version in installed_blender_version_list {
        if version.installation_directory_path.starts_with(&blender_repo_path_entry.repo_directory_path) {
            match installed_blender_version_repository.delete(&version.id).await {
                Ok(_) => {},
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to delete installed Blender version entry: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to delete installed Blender version entry: {:?}", err))
                },
            }
        }
    } 
    match blender_repo_path_repository.delete(&id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to delete Blender repo path entry: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to delete Blender repo path entry: {:?}", err))
        },
    }
}
