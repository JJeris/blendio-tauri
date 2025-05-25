use crate::{
    db_repo::{
        InstalledBlenderVersionRepository, LaunchArgumentRepository, ProjectFileRepository,
        PythonScriptRepository,
    },
    file_system_utility::{self, show_ok_notification},
    models::ProjectFile,
    AppState,
};
use tauri::AppHandle;

/// ID: PF_001
/// ABC analīzes rezultāts:3,24,6
#[tauri::command]
pub async fn insert_blend_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    file_path: std::path::PathBuf,
) -> Result<(), String> {
    let file_name = match file_path.file_name() {
        // A (1.a.) let file_name =; C (3.b.) match; B (2.a.) .file_name()
        Some(val) => val.to_string_lossy().to_string(), // C (3.c) Some(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        None => {
            // C (3.c) None =>;
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert project file: can't identify file name"),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to insert project file: can't identify file name"
            )); // B (2.b.) priekšlaicīgs return
        }
    };
    let entry = ProjectFile {
        // A (1.a.) let entry =;
        id: uuid::Uuid::new_v4().to_string(), // B (2.a.) uuid::Uuid::new_v4(); B (2.a.) .to_string();
        file_path: file_path.to_string_lossy().to_string(), // B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        file_name: file_name,
        associated_series_json: serde_json::to_string(&Vec::<String>::new()).unwrap(), // B (2.a.) ::to_string(); B (2.a.) ...::new(); B (2.a.) .unwrap()
        last_used_blender_version_id: None,
        created: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        modified: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        accessed: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
    };
    let repository = ProjectFileRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    match repository.insert(&entry).await {
        // C (3.b) match; B (2.a.) repository.insert()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert project file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to insert project file: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: PF_002
/// ABC analīzes rezultāts:28,101,50
#[tauri::command]
pub async fn insert_and_refresh_blend_files(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let config_directory = match dirs::config_dir() {
        // A (1.a.) let config_directory =; C (3.b) match
        Some(val) => val, // C (3.c) Some()
        None => {
            // C (3.c) None =>;
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert and refresh project files: no config directory found"),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to insert and refresh project files: no config directory found"
            )); // B (2.b.) priekšlaicīgs return
        }
    };
    let blender_foundation_directory = config_directory.join("Blender Foundation").join("Blender"); // A (1.a.) let blender_foundation_directory =; B (2.a.) .join("Blender Foundation"); B (2.a.) .join("Blender")
                                                                                                    // Read in the recent-files.txt.
    let directory_entries = match std::fs::read_dir(blender_foundation_directory) {
        // A (1.a.) let directory_entries =; C (3.b) match; B (2.a.) ::read_dir()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert and refresh project files: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to insert and refresh project files: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    };
    for entry in directory_entries {
        // A (1.a.) let entry =;
        let entry_dir_entry = match entry {
            // A (1.a.) let entry_dir_entry =; C (3.b) match
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => {
                // C (3.c) Err();
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to insert and refresh project files: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to insert and refresh project files: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        };
        let series_name = entry_dir_entry.file_name().to_string_lossy().to_string(); // A (1.a.) let series_name =; B (2.a.) .file_name(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        let recent_files_txt_path = entry_dir_entry // A (1.a.) let recent_files_txt_path =;
            .path() // B (2.a.) .path()
            .join("config") // B (2.a.) .join()
            .join("recent-files.txt"); // B (2.a.) .join()
        if !recent_files_txt_path.exists() {
            // B (2.a.) .exists(); C (3.a) recent_files_txt_path.exists() != true
            continue; // B (2.b.) continue
        }
        // Read in the file.
        let recent_files_txt_content = match std::fs::read_to_string(&recent_files_txt_path) {
            // A (1.a.) let recent_files_txt_content =; C (3.b) match; B (2.a.) ::read_to_string()
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => {
                // C (3.c) Err();
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to insert and refresh project files: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to insert and refresh project files: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        };
        // Holds only the blend file paths that are confirmed to exist.
        let mut refreshed_recent_files_txt_content = String::new(); // A (1.a.) let mut refreshed_recent_files_txt_content =; B (2.a.) ...::new()
        for line in recent_files_txt_content.lines() {
            // A (1.a.) let line =; B (2.a.) .lines()
            let raw_line = line.trim(); // A (1.a.) let raw_line =; B (2.a.) .trim()
            let file_path = std::path::PathBuf::from(raw_line); // A (1.a.) let file_path =; B (2.a.) ::from()
            if !file_path.exists() {
                // B (2.a.) .exists(); C (3.a) file_path.exists() != true
                let mut current_entries = match repository // A (1.a.) let mut current_entries =; C (3.b) match
                    .fetch(None, None, Some(&file_path.to_string_lossy().to_string())) // B (2.a.) repository.fetch(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
                    .await
                {
                    Ok(val) => val, // C (3.c.) Ok()
                    Err(err) => {
                        // C (3.c) Err();
                        show_ok_notification(
                            // B (2.a.) show_ok_notification()
                            app.clone(), // B (2.a.) app.clone();
                            format!("Failed to insert and refresh project files: {:?}", err),
                            tauri_plugin_dialog::MessageDialogKind::Error,
                        );
                        return Err(format!(
                            "Failed to insert and refresh project files: {:?}",
                            err
                        )); // B (2.b.) priekšlaicīgs return
                    }
                };
                if !current_entries.is_empty() {
                    // B (2.a.) .is_empty(); C (3.a) current_entries.is_empty() != true
                    let entry_to_remove = current_entries.remove(0); // A (1.a.) let entry_to_remove =; B (2.a.) .remove()
                    match repository.delete(&entry_to_remove.id).await {
                        // C (3.b) match; B (2.a.) repository.delete()
                        Ok(_) => {} // C (3.c.) Ok()
                        Err(err) => {
                            // C (3.c) Err();
                            show_ok_notification(
                                // B (2.a.) show_ok_notification()
                                app.clone(), // B (2.a.) app.clone();
                                format!("Failed to delete project file: {:?}", err),
                                tauri_plugin_dialog::MessageDialogKind::Error,
                            );
                            return Err(format!("Failed to delete project file: {:?}", err));
                            // B (2.b.) priekšlaicīgs return
                        }
                    }
                }
                continue; // B (2.b.) continue
            } else {
                // C (3.b.) else
                // Update.
                refreshed_recent_files_txt_content.push_str(&file_path.to_string_lossy()); // A (1.c.) .push_str(); B (2.a.) .to_string_lossy()
                refreshed_recent_files_txt_content.push('\n'); // A (1.c.) .push();
            }
            let mut existing_entries = match repository // A (1.a.) let mut existing_entries =; C (3.b) match
                .fetch(None, None, Some(&file_path.to_string_lossy().to_string())) // B (2.a.) repository.fetch(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
                .await
            {
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err();
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to fetch project files: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!("Failed to fetch project files: {:?}", err));
                    // B (2.b.) priekšlaicīgs return
                }
            };
            if existing_entries.is_empty() {
                // B (2.a.) .is_empty(); C (3.a) existing_entries.is_empty() == true
                let file_name = match file_path.file_name() {
                    // A (1.a.) let file_name =; C (3.b) match; B (2.a.) .file_name();
                    Some(val) => val.to_string_lossy().to_string(), // C (3.c) Some(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
                    None => {
                        // C (3.c) None =>;
                        show_ok_notification(app.clone(), format!("Failed to insert and refresh project file: can't identify file name"), tauri_plugin_dialog::MessageDialogKind::Error); // B (2.a.) show_ok_notification(); B (2.a.) app.clone();
                        return Err(format!(
                            "Failed to insert and refresh project file: can't identify file name"
                        )); // B (2.b.) priekšlaicīgs return
                    }
                };
                // If an entry does not exist, insert it.
                let new_project_file_entry = ProjectFile {
                    // A (1.a.) let new_project_file_entry =
                    id: uuid::Uuid::new_v4().to_string(), // B (2.a.) uuid::Uuid::new_v4(); B (2.a.) .to_string();
                    file_path: file_path.to_string_lossy().to_string(), // B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
                    file_name: file_name,
                    associated_series_json: serde_json::to_string(&vec![series_name.clone()]) // B (2.a.) to_string(); B (2.a.) series_name.clone(); B (2.a.) .unwrap()
                        .unwrap(),
                    last_used_blender_version_id: None,
                    created: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
                    modified: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
                    accessed: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
                };
                match repository.insert(&new_project_file_entry).await {
                    // C (3.b) match; B (2.a.) repository.insert()
                    Ok(_) => {} // C (3.c.) Ok()
                    Err(err) => {
                        // C (3.c) Err();
                        show_ok_notification(
                            // B (2.a.) show_ok_notification()
                            app.clone(), // B (2.a.) app.clone();
                            format!("Failed to insert project file: {:?}", err),
                            tauri_plugin_dialog::MessageDialogKind::Error,
                        );
                        return Err(format!("Failed to insert project file: {:?}", err));
                        // B (2.b.) priekšlaicīgs return
                    }
                }
            } else {
                // C (3.b.) else
                let mut existing_entry = existing_entries.remove(0); // A (1.a.) let mut existing_entry =; B (2.a.) .remove()
                let mut associated_series_json: Vec<String> = // A (1.a.) let mut associated_series_json =;
                    match serde_json::from_str(&existing_entry.associated_series_json) { // C (3.b) match; B (2.a.) ::from_str()
                        Ok(val) => val, // C (3.c.) Ok()
                        Err(err) => { // C (3.c) Err();
                            show_ok_notification( // B (2.a.) show_ok_notification()
                                app.clone(), // B (2.a.) app.clone();
                                format!("Failed to insert and refresh project files: {:?}", err),
                                tauri_plugin_dialog::MessageDialogKind::Error,
                            );
                            return Err(format!(
                                "Failed to insert and refresh project files: {:?}",
                                err
                            )); // B (2.b.) priekšlaicīgs return
                        }
                    };
                if !associated_series_json.contains(&series_name) {
                    // B (2.a.) .contains(); C (3.a) associated_series_json.contains() != true
                    associated_series_json.push(series_name.clone()); // A (1.c.) .push(); B (2.a.) series_name.clone();
                    associated_series_json.sort(); // A (1.c.) .sort();
                    existing_entry.associated_series_json = // A (1.a.) existing_entry.associated_series_json =;
                        serde_json::to_string(&associated_series_json).unwrap(); // B (2.a.) ::to_string(); B (2.a.) .unwrap()
                    match repository.update(&existing_entry).await {
                        // C (3.b) match; B (2.a.) repository.update()
                        Ok(_) => {} // C (3.c.) Ok()
                        Err(err) => {
                            // C (3.c) Err();
                            show_ok_notification(
                                // B (2.a.) show_ok_notification()
                                app.clone(), // B (2.a.) app.clone();
                                format!("Failed to insert and refresh project files: {:?}", err),
                                tauri_plugin_dialog::MessageDialogKind::Error,
                            );
                            return Err(format!(
                                "Failed to insert and refresh project files: {:?}",
                                err
                            )); // B (2.b.) priekšlaicīgs return
                        }
                    }
                }
            }
        }
        // Write refreshed_recent_files_txt_content to recent-files.txt.
        match std::fs::write(recent_files_txt_path, refreshed_recent_files_txt_content) {
            // C (3.b) match; B (2.a.) ::write()
            Ok(_) => {} // C (3.c.) Ok()
            Err(err) => {
                // C (3.c) Err();
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to insert and refresh project files: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to insert and refresh project files: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        }
    }
    let current_entries = match repository.fetch(None, None, None).await {
        // A (1.a.) let current_entries =; C (3.b) match; B (2.a.) repository.fetch()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch project files: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch project files: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    for entry in current_entries {
        // A (1.a.) let entry =;
        let path = std::path::Path::new(&entry.file_path); // A (1.a.) let path =; B (2.a.) ...::new()
        if !path.exists() {
            // B (2.a.) .exists(); C (3.a) path.exists() != true
            match repository.delete(&entry.id).await {
                // C (3.b) match; B (2.a.) repository.delete()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err();
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to delete project file entry: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!("Failed to delete project file entry: {:?}", err));
                    // B (2.b.) priekšlaicīgs return
                }
            }
        }
    }
    Ok(())
}

/// ID: PF_003
/// ABC analīzes rezultāts:3,8,3
#[tauri::command]
pub async fn fetch_blend_files(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    file_path: Option<String>,
) -> Result<Vec<ProjectFile>, String> {
    let repository = ProjectFileRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository // A (1.a.) let mut results =; C (3.b) match
        .fetch(id.as_deref(), limit, file_path.as_deref()) // B (2.a.) repository.fetch(); B (2.a.) .as_deref(); B (2.a.) .as_deref();
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch project files: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch project files: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    // Sort DESC
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed)); // A (1.c.) .sort_by(); B (2.a) |a, b| b.accessed.cmp(&a.accessed)
    Ok(results)
}

/// ID: PF_004
/// ABC analīzes rezultāts:4,24,13
#[tauri::command]
pub async fn delete_blend_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let confirmation = file_system_utility::show_ask_notification(
        // A (1.a.) let confirmation =; B (2.a.) show_ask_notification()
        app.clone(), // B (2.a.) app.clone();
        format!("Are you sure you want to delete this .blend file?"),
        tauri_plugin_dialog::MessageDialogKind::Warning,
    );
    if confirmation == false {
        // C (3.a.) confirmation == false
        return Ok(()); // B (2.b.) priekšlaicīgs return
    }
    let repository = ProjectFileRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut project_file_list = match repository // A (1.a.) let mut results =; // C (3.b.) match
        .fetch(id.as_deref(), None, None) // B (2.a.) repository.fetch(); B (2.a.) id.as_deref();
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch project files: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch project files: {:?}", err)); // B(2.b) priekšlaicīgs return
        }
    };
    let entry = project_file_list.remove(0); // A (1.a.) let entry =; B (2.a.) project_file_list.remove();
    match file_system_utility::delete_file(std::path::PathBuf::from(entry.file_path)).await {
        // C (3.b.) match; B (2.a.) ...::delete_file(); B (2.a.) PathBuf::from()
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            match repository.delete(&id.unwrap()).await {
                // C (3.b.) match; B (2.a.) repository.delete(); B (2.a.) id.unwrap()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) =>
                // C (3.c) Err()
                {
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to delete project file: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    )
                }
            }
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete blend file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to delete blend file: {:?}", err)); // B(2.b) priekšlaicīgs return
        }
    }
    match repository.delete(&id.unwrap()).await {
        // C (3.b.) match; B (2.a.) repository.delete(); B (2.a.) id.unwrap()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete project file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to delete project file: {:?}", err)); // B(2.b) priekšlaicīgs return
        }
    }
}

/// ID: PF_005
/// ABC analīzes rezultāts:19,74,40
#[tauri::command]
pub async fn open_blend_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    installed_blender_version_id: String,
    launch_arguments_id: Option<String>,
    python_script_id: Option<String>,
) -> Result<(), String> {
    // Update project file last used Blender version.
    let project_file_repository = ProjectFileRepository::new(&state.pool); // A (1.a.) let project_file_repository =; B (2.a.) ...::new()
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let installed_blender_version_repository =; B (2.a.) ...::new()
    let launch_argument_repository = LaunchArgumentRepository::new(&state.pool); // A (1.a.) let launch_argument_repository =; B (2.a.) ...::new()
    let python_script_repository = PythonScriptRepository::new(&state.pool); // A (1.a.) let python_script_repository =; B (2.a.) ...::new()
    let mut project_file_entry_list = // A (1.a.) let mut project_file_entry_list =;
        match project_file_repository.fetch(Some(&id), None, None).await { // C (3.b) match; B (2.a.) project_file_repository.fetch()
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => { // C (3.c) Err();
                show_ok_notification( // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to fetch project files: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!("Failed to fetch project files: {:?}", err));
                // B (2.b.) priekšlaicīgs return
            }
        };
    if project_file_entry_list.is_empty() {
        // C (3.a) project_file_entry_list.is_empty() == true; B (2.a.) .is_empty()
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch project file by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch project file by ID")); // B (2.b.) priekšlaicīgs return
    }
    let mut project_file_entry = project_file_entry_list.remove(0); // A (1.a.) let mut project_file_entry =; B (2.a.) project_file_entry_list.remove()
    project_file_entry.last_used_blender_version_id = Some(installed_blender_version_id.clone()); // A (1.a.) project_file_entry.last_used_blender_version_id =; B (2.a.) installed_blender_version_id.clone();
    match project_file_repository.update(&project_file_entry).await {
        // C (3.b) match; B (2.a.) project_file_repository.update()
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to update project file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to update project file: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
    let mut installed_blender_version_list = match installed_blender_version_repository // A (1.a.) let mut installed_blender_version_list =; C (3.b) match
        .fetch(Some(&installed_blender_version_id.clone()), None, None) // B (2.a.) installed_blender_version_repository.fetch(); B (2.a.) installed_blender_version_id.clone();
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch installed Blender versions: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to fetch installed Blender versions: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    };
    if installed_blender_version_list.is_empty() {
        // C (3.a.) installed_blender_version_list.is_empty() == true; B (2.a.) .is_empty()
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch installed Blender version by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch installed Blender version by ID")); // B (2.b.) priekšlaicīgs return
    }
    let installed_blender_version_entry = installed_blender_version_list.remove(0); // A (1.a.) let installed_blender_version_entry =; B (2.a.) installed_blender_version_list.remove()
    match installed_blender_version_repository // C (3.b.) match
        .update(&installed_blender_version_entry) // B (2.a.) installed_blender_version_repository.update()
        .await
    {
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to update installed Blender version: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to update project file: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
    let mut final_launch_args: Vec<String> = vec![]; // A (1.a.) let mut final_launch_args =
    final_launch_args.push(project_file_entry.file_path.clone()); // A (1.c.) final_launch_args.push(); B (2.a.) project_file_entry.file_path.clone();
    match launch_arguments_id {
        // C (3.b) match
        Some(arg_id) => {
            // C (3.c) Some()
            let mut launch_argument_entry_list = match launch_argument_repository // A (1.a.) let mut launch_argument_entry_list =; C (3.b) match
                .fetch(Some(&arg_id), None, None) // B (2.a.) launch_argument_repository.fetch()
                .await
            {
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err();
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to fetch launch arguments: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!("Failed to fetch launch arguments: {:?}", err));
                    // B (2.b.) priekšlaicīgs return
                }
            };
            if launch_argument_entry_list.is_empty() {
                // C (3.a.) launch_argument_entry_list.is_empty() == true; B (2.a.) .is_empty()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to fetch launch argument by ID"),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!("Failed to fetch launch argument by ID")); // B (2.b.) priekšlaicīgs return
            }
            let entry = launch_argument_entry_list.remove(0); // A (1.a.) let entry =; B (2.a.) launch_argument_entry_list.remove()
            match launch_argument_repository.update(&entry).await {
                // C (3.b) match; B (2.a.) launch_argument_repository.update()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err();
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to update launch argument: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!("Failed to update launch argument: {:?}", err));
                    // B (2.b.) priekšlaicīgs return
                }
            }
            let parsed_args: Vec<String> = entry // A (1.a.) let parsed_args =
                .argument_string
                .split_whitespace() // B (2.a.) .split_whitespace()
                .map(|s| s.to_string()) // B (2.a.) .map(); B (2.a.) .to_string()
                .collect(); // B (2.a.) .collect()
            final_launch_args.extend(parsed_args); // B (2.a.) .extend()
        }
        None => {} // C (3.c) None =>;
    }
    match python_script_id {
        // C (3.b) match
        Some(script_id) => {
            // C (3.c) Some()
            let mut python_script_entry_list = match python_script_repository // A (1.a.) let mut python_script_entry_list =; C (3.b) match
                .fetch(Some(&script_id), None, None) // B (2.a.) python_script_repository.fetch()
                .await
            {
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err();
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to fetch python scripts: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!("Failed to fetch python scripts: {:?}", err));
                    // B (2.b.) priekšlaicīgs return
                }
            };
            if python_script_entry_list.is_empty() {
                // C (3.a) python_script_entry_list.is_empty() == true; B (2.a.) .is_empty()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to fetch python script by ID"),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!("Failed to fetch python script by ID")); // B (2.b.) priekšlaicīgs return
            }
            let entry = python_script_entry_list.remove(0); // A (1.a.) let entry =; B (2.a.) python_script_entry_list.remove()
            match python_script_repository.update(&entry).await {
                // C (3.b) match; B (2.a.) python_script_repository.update()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err();
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to update python script: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!("Failed to update python script: {:?}", err));
                    // B (2.b.) priekšlaicīgs return
                }
            }
            if !final_launch_args.contains(&"--python".to_string()) {
                // C (3.a.) final_launch_args.contains(&"--python".to_string()) != true; B (2.a.) .contains(); B (2.a.) to_string()
                final_launch_args.push("--python".to_string()); // A (1.c.) .push(); B (2.a.) .to_string()
                final_launch_args.push(entry.script_file_path); // A (1.c.) .push()
            } else if final_launch_args.contains(&"--python".to_string()) {
                // C (3.b.) else; C (3.a.) final_launch_args.contains(&"--python".to_string()) == true; B (2.a.) .contains(); B (2.a.) .to_string()
                final_launch_args.push(entry.script_file_path); // A (1.c.) .push()
            }
        }
        None => {} // C (3.c) None =>;
    }
    match file_system_utility::launch_executable(
        // C (3.b) match; B (2.a.) ::launch_executable()
        installed_blender_version_entry.executable_file_path.into(), // B (2.a.) .into()
        Some(final_launch_args),
    ) {
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to open project file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to open project file: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: PF_006
/// ABC analīzes rezultāts:8,25,16
#[tauri::command]
pub async fn create_new_project_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    installed_blender_version_id: String,
    mut file_name: String,
) -> Result<(), String> {
    let directory_path_option = // A (1.a.) let directory_path_option =
        match file_system_utility::get_directory_from_file_explorer(app.clone()) // C (3.b) match; B (2.a.) get_directory_from_file_explorer(); B (2.a.) app.clone();
            .await
        {
            Ok(val) => val, // C (3.c.) Ok()
            Err(_) => return Ok(()), // C (3.c.) Err(); B (2.b.) priekšlaicīgs return
        };
    let directory_path = match directory_path_option {
        // A (1.a.) let directory_path =; C (3.b) match
        Some(val) => val, // C (3.c) Some()
        None => {
            // C (3.c.) None =>;
            return Ok(()); // B (2.b.) priekšlaicīgs return
        }
    };
    if !file_name.ends_with(".blend") {
        // C (3.a.) file_name.ends_with() != true; B (2.a.) .ends_with()
        file_name = format!("{}.blend", file_name); // A (1.a.) file_name =;
    }
    let full_file_path = directory_path.join(&file_name); // A (1.a.) let full_file_path =; B (2.a.) .join()
    let python_code_expression = format!(
        // A (1.a.) let python_code_expression =;
        r#"
{}
blend_file_path=r"{}"
{}
"#,
        super::IMPORT_BPY,
        full_file_path.display(), // B (2.a.) full_file_path.display()
        super::SAVE_AS_MAINFILE
    );
    let repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut entry_list = match repository // A (1.a.) let mut entry_list =; C (3.b) match
        .fetch(Some(&installed_blender_version_id), None, None) // B (2.a.) repository.fetch()
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch installed Blender versions: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to fetch installed Blender versions: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    };
    let entry = entry_list.remove(0); // A (1.a.) et entry =; B (2.a.) entry_list.remove()
    match file_system_utility::launch_executable(
        // C (3.b) match; B (2.a.) ::launch_executable()
        entry.executable_file_path.into(), // B (2.a.) .into()
        Some(vec![
            "--background".to_string(),  //  B (2.a.) .to_string()
            "--python-expr".to_string(), //  B (2.a.) .to_string()
            python_code_expression,
        ]),
    ) {
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to launch Blender version: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to launch Blender version: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
    match insert_blend_file(app.clone(), state, full_file_path).await {
        // C (3.b) match; B (2.a.) insert_blend_file(); B (2.a.) app.clone();
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert project file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to insert project file: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: PF_007
/// ABC analīzes rezultāts:3,12,6
#[tauri::command]
pub async fn reveal_project_file_in_local_file_system(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut project_file_list = match repository.fetch(id.as_deref(), None, None).await {
        // A (1.a.) let mut project_file_list =; C (3.b) match; B (2.a.) repository.fetch(); B (2.a.) .as_deref();
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch project files: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch project files: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    let entry = project_file_list.remove(0); // A (1.a.) let entry =; B (2.a.) project_file_list.remove()
    match file_system_utility::open_in_file_explorer(std::path::PathBuf::from(entry.file_path)) {
        // C (3.b) match; B (2.a.) ::open_in_file_explorer(); B (2.a.) ::from()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to open project file in file explorer: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to open project file in file explorer: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: PF_008
/// ABC analīzes rezultāts:4,17,9
#[tauri::command]
pub async fn create_project_file_archive_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let repository = ProjectFileRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new();
    let mut entry_list = match repository.fetch(id.as_deref(), None, None).await {
        // A (1.a.) let mut entry_list =; C (3.b) match; B (2.a.) repository.fetch(); B (2.a.) .as_deref()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch project files: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch project files: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    let entry = entry_list.remove(0); // A (1.a.) let entry =; B (2.a.) entry_list.remove();
    let archive_path = match file_system_utility::archive_file(std::path::PathBuf::from(
        // A (1.a.) let archive_path =; C (3.b) match; B (2.a.) ::archive_file(); B (2.a.) ::from()
        entry.file_path.clone(), // B (2.a.) entry.file_path.clone();
    )) {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to archive project file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to archive project file: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    match file_system_utility::open_in_file_explorer(archive_path) {
        // C (3.b) match; B (2.a.) ::open_in_file_explorer()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c.) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!(
                    "Failed to open project archive file in file explorer: {:?}",
                    err
                ),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to open project archive file in file explorer: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
}
