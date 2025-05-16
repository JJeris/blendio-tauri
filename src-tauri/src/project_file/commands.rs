use chrono::Utc;
use tauri::{AppHandle};
use uuid::Uuid;
use crate::{db_repo::{InstalledBlenderVersionRepository, LaunchArgumentRepository, ProjectFileRepository, PythonScriptRepository}, file_system_utility::{self, show_ok_notification}, models::{InstalledBlenderVersion, LaunchArgument, ProjectFile}, AppState};

/// ID: PF_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn insert_blend_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    file_path: std::path::PathBuf
) -> Result<(), String> {
    let file_name = match file_path.file_name() {
        Some(val) => val.to_string_lossy().to_string(),
        None => {
            show_ok_notification(app.clone(), format!("Failed to insert project file: can't identify file name"), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert project file: can't identify file name"))
        },
    };
    let entry = ProjectFile {
        id: uuid::Uuid::new_v4().to_string(),
        file_path: file_path.to_string_lossy().to_string(), 
        file_name: file_name,
        associated_series_json: serde_json::to_string(&Vec::<String>::new()).unwrap(),
        last_used_blender_version_id: None,
        created: chrono::Utc::now().to_rfc3339(),
        modified: chrono::Utc::now().to_rfc3339(),
        accessed: chrono::Utc::now().to_rfc3339(), 
    };
    let repository = ProjectFileRepository::new(&state.pool);
    match repository.insert(&entry).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to insert project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert project file: {:?}", err))
        },
    }
}

/// ID: PF_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn insert_and_refresh_blend_files(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let config_directory = match dirs::config_dir() {
        Some(val) => val,
        None => {
            show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: no config directory found"), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert and refresh project files: no config directory found"))
        },
    };
    let blender_foundation_directory = config_directory.join("Blender Foundation").join("Blender");
    // Read in the recent-files.txt.
    let directory_entries = match std::fs::read_dir(blender_foundation_directory) {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert and refresh project files: {:?}", err))
        },
    };
    for entry in directory_entries {
        let entry_dir_entry = match entry {
            Ok(val) => val,
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to insert and refresh project files: {:?}", err))
            },
        };
        let series_name = entry_dir_entry.file_name().to_string_lossy().to_string(); 
        let recent_files_txt_path = entry_dir_entry.path().join("config").join("recent-files.txt");
        if !recent_files_txt_path.exists()
        {
            continue;
        }
        // Read in the file.
        let recent_files_txt_content = match std::fs::read_to_string(&recent_files_txt_path) {
            Ok(val) => val,
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to insert and refresh project files: {:?}", err))
            },
        };
        // Holds only the blend file paths that are confirmed to exist.
        let mut refreshed_recent_files_txt_content = String::new(); 
        for line in recent_files_txt_content.lines() {
            let raw_line = line.trim();
            let file_path = std::path::PathBuf::from(raw_line);
            if !file_path.exists()
            {
                let mut current_entries = match repository.fetch(None, None, Some(&file_path.to_string_lossy().to_string())).await {
                    Ok(val) => val,
                    Err(err) => {
                        show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                        return Err(format!("Failed to insert and refresh project files: {:?}", err))
                    },
                };
                if !current_entries.is_empty()
                {
                    let entry_to_remove = current_entries.remove(0);
                    match repository.delete(&entry_to_remove.id).await {
                        Ok(_) => {},
                        Err(err) => {
                            show_ok_notification(app.clone(), format!("Failed to delete project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                            return Err(format!("Failed to delete project file: {:?}", err))
                        },
                    }
                }
                continue;
            }
            else 
            {
                // Update.
                refreshed_recent_files_txt_content.push_str(&file_path.to_string_lossy());
                refreshed_recent_files_txt_content.push('\n');
            }
            let mut existing_entries = match repository.fetch(None, None, Some(&file_path.to_string_lossy().to_string())).await {
                Ok(val) => val,
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to fetch project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to fetch project files: {:?}", err))
                },
            };
            if existing_entries.is_empty()
            {
                let file_name = match file_path.file_name() {
                    Some(val) => val.to_string_lossy().to_string(),
                    None => {
                        show_ok_notification(app.clone(), format!("Failed to insert and refresh project file: can't identify file name"), tauri_plugin_dialog::MessageDialogKind::Error);
                        return Err(format!("Failed to insert and refresh project file: can't identify file name"))
                    },
                };
                // If an entry does not exist, insert it.
                let new_project_file_entry = ProjectFile { 
                    id: uuid::Uuid::new_v4().to_string(), 
                    file_path: file_path.to_string_lossy().to_string(), 
                    file_name: file_name, 
                    associated_series_json: serde_json::to_string(&vec![series_name.clone()]).unwrap(), 
                    last_used_blender_version_id: None, 
                    created: chrono::Utc::now().to_rfc3339(),
                    modified: chrono::Utc::now().to_rfc3339(),
                    accessed: chrono::Utc::now().to_rfc3339(), 
                };
                match repository.insert(&new_project_file_entry).await {
                    Ok(_) => {},
                    Err(err) => {
                        show_ok_notification(app.clone(), format!("Failed to insert project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                        return Err(format!("Failed to insert project file: {:?}", err))
                    },
                }
            }
            else
            {
                // If an entry with file_path exists, then update the associated series.
                let mut existing_entry = existing_entries.remove(0);
                let mut associated_series_json: Vec<String> = match serde_json::from_str(&existing_entry.associated_series_json) {
                    Ok(val) => val,
                    Err(err) => {
                        show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                        return Err(format!("Failed to insert and refresh project files: {:?}", err))
                    },
                };
                if !associated_series_json.contains(&series_name)
                {
                    associated_series_json.push(series_name.clone());
                    associated_series_json.sort();
                    existing_entry.associated_series_json = serde_json::to_string(&associated_series_json).unwrap();
                    match repository.update(&existing_entry).await {
                        Ok(_) => {},
                        Err(err) => {
                            show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                            return Err(format!("Failed to insert and refresh project files: {:?}", err))
                        },
                    }
                }
            }
        }
        // Write refreshed_recent_files_txt_content to recent-files.txt.
        match std::fs::write(recent_files_txt_path, refreshed_recent_files_txt_content) {
            Ok(_) => {},
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to insert and refresh project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to insert and refresh project files: {:?}", err))
            },
        }
    }
    let current_entries = match repository.fetch(None, None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch project files: {:?}", err));
        },
    };
    for entry in current_entries {
        let path = std::path::Path::new(&entry.file_path);
        if !path.exists() {
            match repository.delete(&entry.id).await {
                Ok(_) => {},
                Err(err) => {
                    show_ok_notification(app.clone(), format!("Failed to delete project file entry: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                    return Err(format!("Failed to delete project file entry: {:?}", err));
                },
            }
        }
    }
    Ok(())
}

/// ID: PF_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn fetch_blend_files(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    file_path: Option<String>
) -> Result<Vec<ProjectFile>, String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let mut results = match repository.fetch(id.as_deref(), limit, file_path.as_deref()).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch project files: {:?}", err))
        },
    };
    // Sort DESC
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed));
    Ok(results)
}

/// ID: PF_004
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn delete_blend_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let confirmation= file_system_utility::show_ask_notification(app.clone(), format!("Are you sure you want to delete this .blend file?"), tauri_plugin_dialog::MessageDialogKind::Warning);
    if confirmation == false {
        return Ok(());
    }
    let repository = ProjectFileRepository::new(&state.pool);
    let mut project_file_list = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch project files: {:?}", err))
        },
    };
    let entry = project_file_list.remove(0);
    match file_system_utility::delete_file(std::path::PathBuf::from(entry.file_path)).await {
        Ok(_) => {},
        Err(err) => {
            match repository.delete(&id.unwrap()).await {
                Ok(_) => {},
                Err(err) => show_ok_notification(app.clone(), format!("Failed to delete project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error),
            }
            show_ok_notification(app.clone(), format!("Failed to delete blend file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to delete blend file: {:?}", err))
        },
    }
    match repository.delete(&id.unwrap()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to delete project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to delete project file: {:?}", err))
        },
    }
}

/// ID: PF_005
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn open_blend_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    installed_blender_version_id: String,
    launch_arguments_id: Option<String>,
    python_script_id: Option<String>
) -> Result<(), String> {
    // Update project file last used Blender version.
    let project_file_repository = ProjectFileRepository::new(&state.pool);
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool);
    let launch_argument_repository = LaunchArgumentRepository::new(&state.pool);
    let python_script_repository = PythonScriptRepository::new(&state.pool);
    let mut project_file_entry_list = match project_file_repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch project files: {:?}", err))
        },
    };
    if project_file_entry_list.is_empty() {
        show_ok_notification(app.clone(), format!("Failed to fetch project file by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
        return Err(format!("Failed to fetch project file by ID"));
    }
    let mut project_file_entry = project_file_entry_list.remove(0);
    project_file_entry.last_used_blender_version_id = Some(installed_blender_version_id.clone());
    match project_file_repository.update(&project_file_entry).await {
        Ok(_) => {},
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to update project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to update project file: {:?}", err))
        },
    }
    let mut installed_blender_version_list = match installed_blender_version_repository.fetch(Some(&installed_blender_version_id.clone()), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
        },
    };
    if installed_blender_version_list.is_empty() {
        show_ok_notification(app.clone(), format!("Failed to fetch installed Blender version by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
        return Err(format!("Failed to fetch installed Blender version by ID"));
    }
    let installed_blender_version_entry = installed_blender_version_list.remove(0);
    match installed_blender_version_repository.update(&installed_blender_version_entry).await {
        Ok(_) => {},
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to update installed Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to update project file: {:?}", err))
        },
    }
    let mut final_launch_args: Vec<String> = vec![];
    final_launch_args.push(project_file_entry.file_path.clone());
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
                return Err(format!("Failed to fetch launch argument by ID"));
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
                return Err(format!("Failed to fetch python script by ID"));
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
    match file_system_utility::launch_executable(installed_blender_version_entry.executable_file_path.into(), Some(final_launch_args)) {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to open project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to open project file: {:?}", err))
        },
    }
}

/// ID: PF_006
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn create_new_project_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    installed_blender_version_id: String,
    mut file_name: String
) -> Result<(), String> {
    let directory_path_option = match file_system_utility::get_directory_from_file_explorer(app.clone(), state.clone()).await {
        Ok(val) => val,
        Err(_) => return Ok(()),
    };
    let directory_path = match directory_path_option {
        Some(val) => val,
        None => {
            // User cancelled dialog or no path selected
            return Ok(())
        },
    };
    if !file_name.ends_with(".blend") {
        file_name = format!("{}.blend", file_name);
    }
    let full_file_path = directory_path.join(&file_name);
    let python_code_expression = format!(
r#"
{}
blend_file_path=r"{}"
{}
"#,
super::IMPORT_BPY,
full_file_path.display(),
super::SAVE_AS_MAINFILE
    );
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut entry_list = match repository.fetch(Some(&installed_blender_version_id), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch installed Blender versions: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch installed Blender versions: {:?}", err))
        },
    };
    let entry = entry_list.remove(0);
    match file_system_utility::launch_executable(entry.executable_file_path.into(), Some(vec!["--background".to_string(), "--python-expr".to_string(), python_code_expression,])) {
        Ok(_) => {},
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to launch Blender version: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to launch Blender version: {:?}", err))
        },
    }
    match insert_blend_file(app.clone(), state, full_file_path).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to insert project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert project file: {:?}", err))
        },
    }
}

/// ID: PF_007
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn reveal_project_file_in_local_file_system(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let mut project_file_list = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch project files: {:?}", err))
        },
    };
    let entry = project_file_list.remove(0);
    match file_system_utility::open_in_file_explorer(std::path::PathBuf::from(entry.file_path)) {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to open project file in file explorer: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to open project file in file explorer: {:?}", err))
        },
    }
}

/// ID: PF_008
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn create_project_file_archive_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool); 
    let mut entry_list = match repository.fetch(id.as_deref(), None, None).await { 
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch project files: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch project files: {:?}", err))
        }, 
    };
    let entry = entry_list.remove(0); 
    let archive_path = match file_system_utility::archive_file(std::path::PathBuf::from(entry.file_path.clone())) { 
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to archive project file: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to archive project file: {:?}", err))
        }, 
    };
    match file_system_utility::open_in_file_explorer(archive_path) { 
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to open project archive file in file explorer: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to open project archive file in file explorer: {:?}", err))
        },
    }
}