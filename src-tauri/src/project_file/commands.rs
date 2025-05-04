//! Contains the frontend exposed commands of the Project file module.

use std::fmt::format;

use chrono::Utc;
use reqwest::header::ValuesMut;
use tauri::{http::version, AppHandle};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::{db_repo::{InstalledBlenderVersionRepository, LaunchArgumentRepository, ProjectFileRepository, PythonScriptRepository}, file_system_utility, models::{InstalledBlenderVersion, LaunchArgument, ProjectFile}, AppState};

/// Saglabāt .blend failu datus
#[tauri::command]
pub async fn insert_blend_file(
    state: tauri::State<'_, AppState>,
    file_path: std::path::PathBuf
) -> Result<(), String> {
    let entry = ProjectFile {
        id: Uuid::new_v4().to_string(),
        file_path: file_path.to_string_lossy().to_string(), 
        file_name: file_path.file_name().unwrap().to_string_lossy().to_string(), // TODO fix.
        associated_series_json: serde_json::to_string(&vec![""]).unwrap(), // TODO fix.
        last_used_blender_version_id: None,
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(), 
    };
    let repository = ProjectFileRepository::new(&state.pool);
    match repository.insert(&entry).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}
// id: Option<String>,

#[tauri::command]
pub async fn update_blend_file(
    state: tauri::State<'_, AppState>,
    id: String,
    associated_series: Option<String>,
    last_used_blender_version_id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let entry_list = match repository.fetch(Some(id.as_str()), None, None).await { 
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if entry_list.is_empty() 
    {
        return Err(String::new());
    }
    let mut entry = entry_list[0].clone(); // TODO fix.
    // Handle associated_series update
    if let Some(series) = associated_series {
        let mut series_list: Vec<String> = serde_json::from_str(&entry.associated_series_json).unwrap_or_default();
        if !series_list.contains(&series) {
            series_list.push(series);
            entry.associated_series_json = serde_json::to_string(&series_list).unwrap_or("[]".into());
        }
    }
    // Handle blender version update
    if let Some(version_id) = last_used_blender_version_id {
        entry.last_used_blender_version_id = Some(version_id);
    }
    // Save updated entry
    match repository.update(&entry).await {
        Ok(_) => Ok(()),
        Err(_) => return Err(String::new())
    }
}

/// Saglabāt .blend failu datus
#[tauri::command]
pub async fn insert_and_refresh_blend_files(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO find the Blender directories. i.e. C:\Users\J\AppData\Roaming\Blender Foundation\Blender
    let repository = ProjectFileRepository::new(&state.pool);
    // Roaming_directory.
    let roaming_directory = match dirs::config_dir() {
        Some(val) => val,
        None => return Err(String::new())
    };
    let blender_foundation_directory = roaming_directory.join("Blender Foundation").join("Blender");
    // Read in the recent-files.txt.
    let directory_entries = match std::fs::read_dir(blender_foundation_directory) {
        Ok(val) => val,
        Err(err) => return Err(String::new())
    };
    for entry in directory_entries {
        let entry_dir_entry = match entry {
            Ok(val) => val,
            Err(err) => return Err(String::new())
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
            Err(err) => return Err(String::new())
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
                    Err(err) => return Err(String::new())
                };
                if !current_entries.is_empty()
                {
                    let entry_to_remove = current_entries.remove(0);
                    match repository.delete(&entry_to_remove.id).await {
                        Ok(_) => {},
                        Err(err) => return Err(String::new())
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
                Err(err) => return Err(String::new())
            };
            if existing_entries.is_empty()
            {
                // If an entry does not exist, insert it.
                let new_project_file_entry = ProjectFile { 
                    id: Uuid::new_v4().to_string(), 
                    file_path: file_path.to_string_lossy().to_string(), 
                    file_name: match file_path.file_name() {
                        Some(val) => val.to_string_lossy().to_string(),
                        None => return Err(String::new()),
                    }, 
                    associated_series_json: serde_json::to_string(&vec![series_name.clone()]).unwrap(), 
                    last_used_blender_version_id: None, 
                    created: Utc::now().to_rfc3339(),
                    modified: Utc::now().to_rfc3339(),
                    accessed: Utc::now().to_rfc3339(), 
                };
                match repository.insert(&new_project_file_entry).await {
                    Ok(_) => {},
                    Err(err) => return Err(String::new())
                }
            }
            else
            {
                // If an entry with file_path exists, then update the associated series.
                let mut existing_entry = existing_entries.remove(0);
                let mut associated_series_json: Vec<String> = match serde_json::from_str(&existing_entry.associated_series_json) {
                    Ok(val) => val,
                    Err(err) => return Err(String::new())
                };
                if !associated_series_json.contains(&series_name)
                {
                    associated_series_json.push(series_name.clone());
                    associated_series_json.sort();
                    existing_entry.associated_series_json = serde_json::to_string(&associated_series_json).unwrap();
                    match repository.update(&existing_entry).await {
                        Ok(_) => {},
                        Err(err) => return Err(String::new())
                    }
                }
            }
        }
        // Write refreshed_recent_files_txt_content to recent-files.txt.
        match std::fs::write(recent_files_txt_path, refreshed_recent_files_txt_content) {
            Ok(_) => {},
            Err(err) => return Err(String::new())
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn fetch_blend_files(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    file_path: Option<String>
) -> Result<Vec<ProjectFile>, String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let results = match repository.fetch(id.as_deref(), limit, file_path.as_deref()).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    Ok(results)
}

/// Izdzēst .blend failu
#[tauri::command]
pub async fn delete_blend_file(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let mut entry_list = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    let entry = entry_list.remove(0);
    file_system_utility::delete_file(std::path::PathBuf::from(entry.file_path)).await?;
    match repository.delete(&id.unwrap()).await { // TODO fix.
        Ok(_) => {},
        Err(err) => return Err(String::new()),
    }
    Ok(())
}

/// Atvērt .blend failu Blender versijā
#[tauri::command]
pub async fn open_blend_file(
    state: tauri::State<'_, AppState>,
    id: String,
    installed_blender_version_id: String,
    launch_arguments_id: Option<String>,
    python_script_id: Option<String>
) -> Result<(), String> {
    // println!("{:?}", id);
    // println!("{:?}", installed_blender_version_id);
    // println!("{:?}", launch_arguments_id);
    // println!("{:?}", python_script_id);
    // Update project file last used Blender version.
    let project_file_repository = ProjectFileRepository::new(&state.pool);
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool);
    let launch_argument_repository = LaunchArgumentRepository::new(&state.pool);
    let python_script_repository = PythonScriptRepository::new(&state.pool);
    let mut project_file_entry_list = match project_file_repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if project_file_entry_list.is_empty() {
        return Err(String::new())
    }
    let project_file_entry = project_file_entry_list.remove(0);
    match project_file_repository.update(&project_file_entry).await {
        Ok(_) => {},
        Err(err) => return Err(String::new()),
    }
    let mut installed_blender_version_list = match installed_blender_version_repository.fetch(Some(&installed_blender_version_id.clone()), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if installed_blender_version_list.is_empty() {
        return Err(String::new())
    }
    let installed_blender_version_entry = installed_blender_version_list.remove(0);
    match installed_blender_version_repository.update(&installed_blender_version_entry).await {
        Ok(_) => {},
        Err(err) => return Err(String::new()),
    }
    let mut final_launch_args: Vec<String> = vec![];
    final_launch_args.push(project_file_entry.file_path.clone());
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
    println!("{:?}, {:?}", installed_blender_version_entry.executable_file_path, final_launch_args);
    match file_system_utility::launch_executable(installed_blender_version_entry.executable_file_path.into(), Some(final_launch_args)) {
        Ok(_) => Ok(()),
        Err(_) => return Err(String::new()),
    }
}

/// Izveidot jaunu .blend failu
#[tauri::command]
pub async fn create_new_project_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    installed_blender_version_id: String,
    mut file_name: String
) -> Result<(), String> {
    let directory_path = app.dialog().file().blocking_pick_folder();
    let file_path_string = match directory_path {
        Some(val) => val.to_string(),
        None => return Err(String::new()),
    };
    let file_path = std::path::PathBuf::from(file_path_string);
    if !file_name.ends_with(".blend") {
        file_name = format!("{}.blend", file_name);
    }
    let full_file_path = file_path.join(&file_name);
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
    // TODO get the blender version.
    let repository = InstalledBlenderVersionRepository::new(&state.pool);
    let mut entry_list = match repository.fetch(Some(&installed_blender_version_id), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    let entry = entry_list.remove(0);
    match file_system_utility::launch_executable(entry.executable_file_path.into(), Some(vec!["--background".to_string(), "--python-expr".to_string(), python_code_expression,])) {
        Ok(_) => {},
        Err(_) => return Err(String::new()),
    }
    match insert_blend_file(state, full_file_path).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}

/// Atrast .blend failu lokālajā failu sistēmā
#[tauri::command]
pub async fn reveal_project_file_in_local_file_system(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let mut entry_list = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    let entry = entry_list.remove(0);
    // entry.file_path 
    match file_system_utility::open_in_file_explorer(std::path::PathBuf::from(entry.file_path)) {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}

/// Saglabāt .blend failus arhīvfailā
#[tauri::command]
pub async fn create_project_file_archive_file(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool);
    let mut entry_list = match repository.fetch(id.as_deref(), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    let entry = entry_list.remove(0);
    let archive_path = match file_system_utility::archive_file(std::path::PathBuf::from(entry.file_path.clone())) {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    match file_system_utility::open_in_file_explorer(archive_path) {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}

// Saglabāt launch argument string vērtību
#[tauri::command]
pub async fn insert_launch_argument(
    state: tauri::State<'_, AppState>,
    argument_string: String,
    project_file_id: Option<String>,
    python_script_id: Option<String>
) -> Result<String, String> {
    let repository = LaunchArgumentRepository::new(&state.pool);
    let mut results = match repository.fetch(None, None, Some(&argument_string)).await {
        Ok(val) => val,
        Err(err) => return Err(format!("Failed to fetch existing args: {:?}", err)),
    };

    if !results.is_empty() {
        let mut existing_entry = results.remove(0);
        existing_entry.accessed = Utc::now().to_rfc3339();
        existing_entry.modified = Utc::now().to_rfc3339();
        match repository.update(&existing_entry).await {
            Ok(_) => return Ok(existing_entry.id),
            Err(err) => return Err(format!("Failed to update entry: {:?}", err)),
        }
    }

    let entry = LaunchArgument{ 
        id: Uuid::new_v4().to_string(), 
        is_default: false, 
        argument_string: argument_string, 
        last_used_project_file_id: project_file_id, 
        last_used_python_script_id: python_script_id, 
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(), 
    };
    match repository.insert(&entry).await {
        Ok(_) => {},
        Err(err) => return Err(format!("{:?}", err)),
    }
    Ok(entry.id)
}

#[tauri::command]
pub async fn fetch_launch_arguments(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    argument_string: Option<String>
) -> Result<Vec<LaunchArgument>, String> {
    let repository = LaunchArgumentRepository::new(&state.pool);
    let mut results = match repository.fetch(id.as_deref(), limit, argument_string.as_deref()).await {
        Ok(val) => val,
        Err(err) => return Err(format!("{:?}", err)),
    };

    results.sort_by(|a, b| b.accessed.cmp(&a.accessed)); // DESC
    Ok(results)
}