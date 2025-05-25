use crate::{
    db_repo::{
        BlenderRepoPathRepository, InstalledBlenderVersionRepository, LaunchArgumentRepository,
        PythonScriptRepository,
    },
    file_system_utility::{self, show_ok_notification},
    models::{BlenderRepoPath, DownloadableBlenderVersion, InstalledBlenderVersion},
    AppState,
};
use regex::Regex;
use tauri::AppHandle;

/// ID: BV_001
/// ABC analīzes rezultāts:10,44,12
#[tauri::command]
pub async fn insert_installed_blender_version(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    executable_file_path: std::path::PathBuf,
) -> Result<(), String> {
    let parent_dir = match executable_file_path.parent() {
        // A (1.a.) let parent_dir =; C (3.b) match; B (2.a.) .parent()
        Some(val) => val, // C (3.c) Some()
        None => {
            // C (3.c) None =>;
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to get file path parent"),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to get file path parent")); // B (2.b.) priekšlaicīgs return
        }
    };
    let dir_name = match parent_dir.file_name() {
        // A (1.a.) let dir_name =; C (3.b) match; B (2.a.) .file_name()
        Some(val) => val.to_string_lossy().to_string(), // C (3.c) Some(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        None => {
            // C (3.c) None =>;
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to get file name"),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to get file name")); // B (2.b.) priekšlaicīgs return
        }
    };
    let re = match Regex::new(r"blender-(?P<version>\d+\.\d+(?:\.\d+)?)-(?P<variant>[^\-+]+)") {
        // A (1.a.) let re =; C (3.b) match; B (2.a.) ...::new()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to construct regex: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to construct regex: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    let mut version = String::new(); // A (1.a.) let mut version =; B (2.a.) ...::new()
    let mut variant_type = String::new(); // A (1.a.) let mut variant_type =; B (2.a.) ...::new()
    if let Some(caps) = re.captures(&dir_name) {
        // A (1.d.) if let Some(); B (2.a.) .captures()
        version = caps // A (1.a.) version = caps
            .name("version") // B (2.a.) .name()
            .map(|m| m.as_str().to_string()) // B (2.a.) .map(); B (2.a.) .as_str(); B (2.a.) .to_string()
            .unwrap_or_default(); // B (2.a.) .unwrap_or_default();
        variant_type = caps // A (1.a.) variant_type = caps
            .name("variant") // B (2.a.) .name()
            .map(|m| m.as_str().to_string()) // B (2.a.) .map(); B (2.a.) .as_str(); B (2.a.) .to_string()
            .unwrap_or_default(); // B (2.a.) .unwrap_or_default();
    }
    // Try to extract version and variant_type using regex
    let entry = InstalledBlenderVersion {
        // A (1.a.) let entry =;
        id: uuid::Uuid::new_v4().to_string(), // B (2.a.) uuid::Uuid::new_v4(); B (2.a.) .to_string();
        version: version,
        variant_type: variant_type,
        download_url: None,
        is_default: false,
        installation_directory_path: parent_dir.to_string_lossy().to_string(), // B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        executable_file_path: executable_file_path.to_string_lossy().to_string(), // B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        created: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        modified: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        accessed: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
    };
    let repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    match repository.insert(&entry).await {
        // C (3.b.) match; B (2.a.) repository.insert();
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert installed Blender version: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to insert installed Blender version: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: BV_002
/// ABC analīzes rezultāts:12,44,25
#[tauri::command]
pub async fn insert_and_refresh_installed_blender_versions(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let blender_repo_paths_repo = BlenderRepoPathRepository::new(&state.pool); // A (1.a.) let blender_repo_paths_repo =; B (2.a.) ...::new()
    let blender_repo_paths = match blender_repo_paths_repo.fetch(None, None, None).await {
        // A (1.a.) let blender_repo_paths =; C (3.b) match; B (2.a.) .fetch()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch Blender repo paths: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err));
            // B (2.b.) priekšlaicīgs return
        }
    };
    let installed_blender_version_repo = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let installed_blender_version_repo =; B (2.a.) ...::new()
    for repo_path in blender_repo_paths {
        // A (1.a.) let repo_path =;
        let directory_entries = match std::fs::read_dir(repo_path.repo_directory_path) {
            // A (1.a.) let directory_entries =; C (3.b) match; B (2.a.) ::read_dir()
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => {
                // C (3.c) Err()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!(
                        "Failed to insert and refresh installed Blender version: {:?}",
                        err
                    ),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to insert and refresh installed Blender version: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        };
        for entry in directory_entries {
            // A (1.a.) let entry =;
            let entry = match entry {
                // A (1.a.) let entry =; C (3.b) match
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!(
                            "Failed to insert and refresh installed Blender version: {:?}",
                            err
                        ),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!(
                        "Failed to insert and refresh installed Blender version: {:?}",
                        err
                    )); // B (2.b.) priekšlaicīgs return
                }
            };
            if !entry.path().is_dir() {
                // C (3.a.) entry.path().is_dir() != true; B (2.a.) .path(); B (2.a.) .is_dir();
                continue; // B (2.b.) continue
            }
            let launcher_path = entry.path().join("blender-launcher.exe"); // A (1.a.) let launcher_path =; B (2.a.) .path(); B (2.a.) .join()
            if !launcher_path.exists() {
                // C (3.a.) launcher_path.exists() != true; B (2.a.) .exists()
                continue; // B (2.b.) continue
            }
            let existing_entries = match installed_blender_version_repo // A (1.a.) let existing_entries =; C (3.b) match
                .fetch(
                    None,
                    None,
                    Some(&launcher_path.to_string_lossy().to_string()), // B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
                ) // B (2.a.) installed_blender_version_repo.fetch()
                .await
            {
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
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
            if !existing_entries.is_empty() {
                // C (3.a.) existing_entries.is_empty() != true; B (2.a.) .exists()
                continue; // B (2.b.) continue
            }
            match insert_installed_blender_version(app.clone(), state.clone(), launcher_path).await // C (3.b) match; B (2.a.) insert_installed_blender_version(); B (2.a.) app.clone(); B (2.a.) state.clone();
            {
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => { // C (3.c) Err()
                    show_ok_notification( // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!(
                            "Failed to insert and refresh installed Blender version: {:?}",
                            err
                        ),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!(
                        "Failed to insert and refresh installed Blender version: {:?}",
                        err
                    )); // B (2.b.) priekšlaicīgs return
                }
            }
        }
    }
    let current_entries = match installed_blender_version_repo.fetch(None, None, None).await {
        // A (1.a.) let current_entries =; C (3.b) match; B (2.a.) installed_blender_version_repo.fetch()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
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
    for entry in current_entries {
        // A (1.a.) let entry =;
        let path = std::path::Path::new(&entry.executable_file_path); // A (1.a.) let path =; B (2.a.) ...::new()
        if !path.exists() {
            // C (3.a.) path.exists() != true; B (2.a.) .exists()
            match installed_blender_version_repo.delete(&entry.id).await {
                // C (3.b) match; B (2.a.) installed_blender_version_repo.delete()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!("Failed to delete Blender version entry: {:?}", err),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!("Failed to delete Blender version entry: {:?}", err));
                    // B (2.b.) priekšlaicīgs return
                }
            }
        }
    }
    Ok(())
}

/// ID: BV_003
/// ABC analīzes rezultāts:8,22,17
#[tauri::command]
pub async fn update_installed_blender_version(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository.fetch(Some(&id), None, None).await {
        // A (1.a.) let mut results =; C (3.b) match
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
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
    if results.is_empty() {
        // C (3.a) results.is_empty() == true; B (2.a.) .is_empty()
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch installed Blender version by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch installed Blender version by ID")); // B (2.b.) priekšlaicīgs return
    }
    let mut entry = results.remove(0); // A (1.a.) let mut entry =; B (2.a.) .remove()
    if is_default == true {
        // C (3.a) is_default == true
        entry.is_default = false; // A (1.a.) entry.is_default =;
        match repository.update(&entry).await {
            // C (3.b) match; B (2.a.) .update()
            Ok(_) => Ok(()), // C (3.c.) Ok(); B (2.b.) priekšlaicīgs return
            Err(err) => {
                // C (3.c) Err()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!(
                        "Failed to update existing installed Blender version: {:?}",
                        err
                    ),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to update existing installed Blender version: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        }
    } else {
        // C (3.b.) else
        let results = match repository.fetch(None, None, None).await {
            // A (1.a.) let results =; C (3.b) match; B (2.a.) .fetch(
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => {
                // C (3.c) Err()
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
        for mut entry in results {
            // A (1.a.) let mut entry =;
            let new_default = entry.id == id; // A (1.a.) let new_default =; C (3.a) entry.id == id
            if entry.is_default != new_default {
                // C (3.a) entry.is_default != new_default
                entry.is_default = new_default; // A (1.a.) entry.is_default =
                match repository.update(&entry).await {
                    // C (3.b) match; B (2.a.) .update()
                    Ok(_) => {} // C (3.c.) Ok()
                    Err(err) => {
                        // C (3.c) Err()
                        show_ok_notification(
                            // B (2.a.) show_ok_notification()
                            app.clone(), // B (2.a.) app.clone();
                            format!("Failed to update installed Blender versions: {:?}", err),
                            tauri_plugin_dialog::MessageDialogKind::Error,
                        );
                        return Err(format!(
                            "Failed to update installed Blender versions: {:?}",
                            err
                        )); // B (2.b.) priekšlaicīgs return
                    }
                }
            }
        }
        Ok(())
    }
}

/// ID: BV_004
/// ABC analīzes rezultāts:3,8,3
#[tauri::command]
pub async fn fetch_installed_blender_versions(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    executable_file_path: Option<&str>,
) -> Result<Vec<InstalledBlenderVersion>, String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.)  B (2.a.) ...::new()
    let mut results = match repository // A (1.a.) let mut results =; C (3.b) match
        .fetch(id.as_deref(), limit, executable_file_path.as_deref()) // B (2.a.) .fetch(); B (2.a.) .as_deref(); B (2.a.) .as_deref();
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
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
    // Sort DESC
    results.sort_by(|a, b| b.version.cmp(&a.version)); // A (1.c.) .sort_by(); B (2.a) |a, b| b.version.cmp(&a.version)
    Ok(results)
}

/// ID: BV_005
/// ABC analīzes rezultāts:4,26,14
#[tauri::command]
pub async fn uninstall_and_delete_installed_blender_version_data(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let confirmation = file_system_utility::show_ask_notification(
        // A (1.a.) let confirmation =; B (2.a.) ::show_ask_notification()
        app.clone(), // B (2.a.) app.clone();
        format!("Are you sure you want to delete this installed Blender version?"),
        tauri_plugin_dialog::MessageDialogKind::Warning,
    );
    if confirmation == false {
        // C (3.a) confirmation == false
        return Ok(()); // B (2.b.) priekšlaicīgs return
    }
    let repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut installed_blender_version_list = match repository.fetch(id.as_deref(), None, None).await // A (1.a.) let mut installed_blender_version_list =; C (3.b) match; B (2.a.) .fetch(); B (2.a.) .as_deref()
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => { // C (3.c) Err()
            show_ok_notification( // B (2.a.) show_ok_notification()
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
        // B (2.a.) .is_empty(); C (3.a) installed_blender_version_list.is_empty() == true
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch installed Blender version by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch installed Blender version by ID")); // B (2.b.) priekšlaicīgs return
    }
    let entry = installed_blender_version_list.remove(0); // A (1.a.) let entry =; B (2.a.) .remove()
    match file_system_utility::delete_directory(std::path::PathBuf::from(
        entry.installation_directory_path,
    )) // C (3.b) match; B (2.a.) ::delete_directory(); B (2.a.) ::from()
    .await
    {
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            match repository.delete(&entry.id).await {
                // C (3.b) match; B (2.a.) .delete()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => show_ok_notification(
                    // C (3.c) Err(); B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to delete installed Blender version: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                ),
            }
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete installed Blender versions: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to delete installed Blender versions: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
    match repository.delete(&entry.id).await {
        // C (3.b) match; B (2.a.) .delete()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete installed Blender version: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to fetch installed Blender version: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: BV_006
/// ABC analīzes rezultāts:15,56,33
#[tauri::command]
pub async fn launch_blender_version_with_launch_args(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    launch_arguments_id: Option<String>,
    python_script_id: Option<String>,
) -> Result<(), String> {
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let installed_blender_version_repository =; B (2.a.) ...::new()
    let launch_argument_repository = LaunchArgumentRepository::new(&state.pool); // A (1.a.) let launch_argument_repository =; B (2.a.) ...::new()
    let python_script_repository = PythonScriptRepository::new(&state.pool); // A (1.a.) let python_script_repository =; B (2.a.) ...::new()
    let mut installed_blender_version_list = match installed_blender_version_repository // A (1.a.) let mut installed_blender_version_list =; C (3.b) match
        .fetch(Some(&id), None, None) // B (2.a.) .fetch()
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
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
        // B (2.a.) .is_empty(); C (3.a) installed_blender_version_list.is_empty() == true
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch installed Blender version by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch installed Blender version by ID")); // B (2.b.) priekšlaicīgs return
    }
    let instance = installed_blender_version_list.remove(0); // A (1.a.) let instance =; B (2.a.) .remove()
    match installed_blender_version_repository.update(&instance).await {
        // C (3.b) match; B (2.a.) .update()
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to update installed Blender version: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to update installed Blender version: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
    let mut final_launch_args: Vec<String> = vec![]; // A (1.a.) let mut final_launch_args =;
    match launch_arguments_id {
        // C (3.b) match
        Some(arg_id) => {
            // C (3.c) Some()
            let mut launch_argument_entry_list = match launch_argument_repository // A (1.a.) let mut launch_argument_entry_list =; C (3.b) match
                .fetch(Some(&arg_id), None, None) // B (2.a.) .fetch()
                .await
            {
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
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
                // B (2.a.) .is_empty(); C (3.a) launch_argument_entry_list.is_empty() == true
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to fetch launch argument by ID"),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!("Failed to fetch launch argument by ID")); // B (2.b.) priekšlaicīgs return
            }
            let entry = launch_argument_entry_list.remove(0); // A (1.a.) let entry =; B (2.b.) .remove()
            match launch_argument_repository.update(&entry).await {
                // C (3.b) match; B (2.b.) .update()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
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
            let parsed_args: Vec<String> = entry // A (1.a.) let parsed_args =;
                .argument_string
                .split_whitespace() // B (2.a.) .split_whitespace()
                .map(|s| s.to_string()) // B (2.a.) .map(_; B (2.a.) .to_string()
                .collect(); // B (2.a.) .collect()
            final_launch_args.extend(parsed_args); // A (1.c.) .extend()
        }
        None => {} // C (3.c) None =>;
    }
    match python_script_id {
        // C (3.b) match
        Some(script_id) => {
            // C (3.c) Some()
            let mut python_script_entry_list = match python_script_repository // A (1.a.) let mut python_script_entry_list =; C (3.b) match
                .fetch(Some(&script_id), None, None) // B (2.a.) .fetch()
                .await
            {
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
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
                // B (2.a.) .is_empty(); C (3.a) python_script_entry_list.is_empty() == true
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to fetch python script by ID"),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!("Failed to fetch python script by ID")); // B (2.b.) priekšlaicīgs return
            }
            let entry = python_script_entry_list.remove(0); // A (1.a.) let entry =; B (2.a.) .remove()
            match python_script_repository.update(&entry).await {
                // C (3.b) match; B (2.a.) .update()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
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
        std::path::PathBuf::from(instance.executable_file_path), // B (2.a.) ::from()
        Some(final_launch_args),
    ) {
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to launch installed Blender version: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to launch installed Blender version: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: BV_007
/// ABC analīzes rezultāts:5,20,18
#[tauri::command]
pub async fn get_downloadable_blender_version_data(
    app: AppHandle,
) -> Result<Vec<DownloadableBlenderVersion>, String> {
    let response = // A (1.a.) let response =;
        match reqwest::get("https://builder.blender.org/download/daily/?format=json&v=2").await { // C (3.b.) match; B (2.a.) ::get()
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => { // C (3.c.) Err()
                show_ok_notification( // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to fetch downloadable Blender versions: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to fetch downloadable Blender versions: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        };
    let response_json: Vec<DownloadableBlenderVersion> = match response.json().await {
        // A (1.a.) let response_json =; C (3.b.) match; B (2.a.) .json()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch downloadable Blender versions: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to fetch downloadable Blender versions: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    };
    #[cfg(target_os = "windows")]
    let filtered_data = response_json // A (1.a.) let filtered_data =;
        .into_iter() // B (2.a.) .into_iter()
        .filter(|p| {
            // B (2.a) |p| {}
            p.bitness == 64 // C (3.a.) p.bitness == 64;
                && p.platform == "windows" // C (3.a.) p.platform == "windows"
                && p.architecture == "amd64" // C (3.a.) p.architecture == "amd64
                && p.file_extension == "zip" // C (3.a.) p.file_extension == "zip"
        }) // B (2.a.) .filter()
        .collect(); // B (2.a.) .collect()

    #[cfg(target_os = "macos")]
    let filtered_data = response_json // A (1.a.) let filtered_data =;
        .into_iter() // B (2.a.) .into_iter()
        .filter(|p| {
            // B (2.a) |p| {}
            p.bitness == 64 // C (3.a.) p.bitness == 64;
                && p.platform == "darwin" // C (3.a.) p.platform == "darwin"
                && p.architecture == "arm64" // C (3.a.) p.architecture == "arm64"
                && p.file_extension == "dmg" // C (3.a.) p.file_extension == "dmg"
        }) // B (2.a.) .filter()
        .collect(); // B (2.a.) .collect()

    #[cfg(target_os = "linux")]
    let filtered_data = response_json // A (1.a.) let filtered_data =;
        .into_iter() // B (2.a.) .into_iter()
        .filter(|p| {
            // B (2.a) |p| {}
            p.bitness == 64 // C (3.a.) p.bitness == 64;
                && p.platform == "linux" // C (3.a.) p.platform == "linux"
                && p.architecture == "x86_64" // C (3.a.) p.architecture == "x86_64"
                && p.file_extension == "xz" // C (3.a.) p.file_extension == "xz"
        }) // B (2.a.) .filter()
        .collect(); // B (2.a.) .collect()
    return Ok(filtered_data);
}

/// ID: BV_008
/// ABC analīzes rezultāts:16,47,17
#[tauri::command]
pub async fn download_and_install_blender_version(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    archive_file_path: std::path::PathBuf,
    downloadable_blender_version: DownloadableBlenderVersion,
) -> Result<(), String> {
    let repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut entry = InstalledBlenderVersion {
        // A (1.a.) let mut entry =;
        id: uuid::Uuid::new_v4().to_string(), // B (2.a.) uuid::Uuid::new_v4(); B (2.a.) .to_string();
        version: downloadable_blender_version.version,
        variant_type: downloadable_blender_version.release_cycle,
        download_url: Some(downloadable_blender_version.url),
        is_default: false,
        installation_directory_path: String::new(), // B (2.a.) ...::new()
        executable_file_path: String::new(),        // B (2.a.) ...::new()
        created: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        modified: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        accessed: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
    };
    let installation_directory_path = match file_system_utility::extract_archive(
        // A (1.a.) let installation_directory_path =; C (3.b) match; B (2.a.) ::extract_archive()
        archive_file_path.clone(), // B (2.a.) .clone();
    )
    .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!(
                    "Failed to extract downloaded Blender versions files from archive file: {:?}",
                    err
                ),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to extract downloaded Blender versions files from archive file: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    };
    entry.installation_directory_path = installation_directory_path.to_string_lossy().to_string(); // A (1.a.) entry.installation_directory_path =; B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
    entry.executable_file_path = installation_directory_path // A (1.a.) entry.executable_file_path =;
        .join("blender-launcher.exe") // B (2.a.) .join()
        .to_string_lossy() // B (2.a.) .to_string_lossy()
        .to_string(); // B (2.a.) .to_string()
    match file_system_utility::delete_file(archive_file_path).await {
        // C (3.b) match; B (2.a.) ::delete_file()
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete downloaded archive file: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to delete downloaded archive file: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
    let mut existing_entries = match repository // A (1.a.) let mut existing_entries =; C (3.b) match
        .fetch(None, None, Some(&entry.executable_file_path)) // B (2.a.) .fetch()
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch Blender repo paths: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err));
            // B (2.b.) priekšlaicīgs return
        }
    };
    if existing_entries.is_empty() {
        // B (2.a.) .is_empty(); C (3.a) existing_entries.is_empty() == true
        match repository.insert(&entry).await {
            // C (3.b.) match; B (2.a.) repository.insert();
            Ok(_) => Ok(()), // C (3.c.) Ok(); B (2.b.) priekšlaicīgs return
            Err(err) => {
                // C (3.c) Err()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to insert installed Blender version: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to insert installed Blender version: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        }
    } else {
        // C (3.b.) else
        let mut old_entry = existing_entries.remove(0); // A (1.a.) let mut old_entry =; B (2.a.) .remove()
        old_entry.version = entry.version; // A (1.a.) old_entry.version =
        old_entry.variant_type = entry.variant_type; // A (1.a.) old_entry.variant_type =
        old_entry.download_url = entry.download_url; // A (1.a.) old_entry.download_url =
        old_entry.is_default = entry.is_default; // A (1.a.) old_entry.is_default =
        old_entry.installation_directory_path = entry.installation_directory_path; // A (1.a.) old_entry.installation_directory_path =
        old_entry.executable_file_path = entry.executable_file_path; // A (1.a.) old_entry.executable_file_path =
        old_entry.created = chrono::Utc::now().to_rfc3339(); // A (1.a.) old_entry.created =; B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        old_entry.modified = chrono::Utc::now().to_rfc3339(); // A (1.a.) old_entry.modified =; B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        old_entry.accessed = chrono::Utc::now().to_rfc3339(); // A (1.a.) old_entry.accessed =; B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        match repository.update(&old_entry).await {
            // C (3.b) match; B (2.a.) .update()
            Ok(_) => Ok(()), // C (3.c.) Ok(); B (2.b.) priekšlaicīgs return
            Err(err) => {
                // C (3.c) Err()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to insert installed Blender version: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to insert installed Blender version: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        }
    }
}

/// ID: BV_009
/// ABC analīzes rezultāts:5,26,13
#[tauri::command]
pub async fn insert_blender_version_installation_location(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let repo_directory_path_option = // A (1.a.) let repo_directory_path_option =
        match file_system_utility::get_directory_from_file_explorer(app.clone()) // C (3.b) match; B (2.a.) get_directory_from_file_explorer(); B (2.a.) app.clone();
            .await
        {
            Ok(val) => val, // C (3.c.) Ok()
            Err(_) => return Ok(()),  // C (3.c) Err(); B (2.b.) priekšlaicīgs return
        };
    let repo_directory_path = match repo_directory_path_option {
        // A (1.a.) let repo_directory_path =; C (3.b) match
        Some(val) => val,      // C (3.c) Some()
        None => return Ok(()), // C (3.c) None =>; B (2.b.) priekšlaicīgs return;
    };
    let repository = BlenderRepoPathRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let results = match repository // A (1.a.) let results =; C (3.b) match
        .fetch(None, None, repo_directory_path.to_str()) // B (2.a.) .fetch(); B (2.a.) .to_str()
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch Blender repo path: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch Blender repo path: {:?}", err));
            // B (2.b.) priekšlaicīgs return
        }
    };
    if !results.is_empty() {
        // B (2.a.) .is_empty(); C (3.a) results.is_empty() != true
        return Ok(()); // B (2.b.) priekšlaicīgs return
    }
    let entry = BlenderRepoPath {
        // A (1.a.) let entry =;
        id: uuid::Uuid::new_v4().to_string(), // B (2.a.) uuid::Uuid::new_v4(); B (2.a.) .to_string();
        repo_directory_path: repo_directory_path.to_string_lossy().to_string(), // B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        is_default: false,
        created: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        modified: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        accessed: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
    };
    match repository.insert(&entry).await {
        // C (3.b.) match; B (2.a.) repository.insert();
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert Blender repo path: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to insert Blender repo path: {:?}", err));
            // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: BV_010
/// ABC analīzes rezultāts:8,23,17
#[tauri::command]
pub async fn update_blender_version_installation_location(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = BlenderRepoPathRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository.fetch(Some(&id), None, None).await {
        // A (1.a.) let mut results =; C (3.b) match; B (2.a.) .fetch()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch Blender repo paths: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err));
            // B (2.b.) priekšlaicīgs return
        }
    };
    if results.is_empty() {
        // B (2.a.) .is_empty(); C (3.a) results.is_empty() == true
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch Blender repo path by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch Blender repo path by ID")); // B (2.b.) priekšlaicīgs return
    }
    let mut entry = results.remove(0); // A (1.a.) let mut entry =; B (2.a.) .remove()
    if is_default == true {
        // C (3.a) is_default == true
        entry.is_default = false; // A (1.a.) entry.is_default =;
        match repository.update(&entry).await {
            // C (3.b) match; B (2.a.) .update()
            Ok(_) => Ok(()), // C (3.c.) Ok(); B (2.b.) priekšlaicīgs return
            Err(err) => {
                // C (3.c) Err()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to update Blender repo path: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!("Failed to update Blender repo path: {:?}", err));
                // B (2.b.) priekšlaicīgs return
            }
        }
    } else {
        // C (3.b.) else
        let results = match repository.fetch(None, None, None).await {
            // A (1.a.) let results =; C (3.b) match; B (2.a.) .fetch()
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => {
                // C (3.c) Err()
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to fetch Blender repo paths: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!("Failed to fetch Blender repo paths: {:?}", err));
                // B (2.b.) priekšlaicīgs return
            }
        };
        for mut entry in results {
            // A (1.a.) let mut entry =;
            let new_default = entry.id == id; // A (1.a.) let new_default =; C (3.a) entry.id == id
            if entry.is_default != new_default {
                // C (3.a) entry.is_default != new_default
                entry.is_default = new_default; // A (1.a.) entry.is_default =;
                match repository.update(&entry).await {
                    // C (3.b) match; B (2.a.) .update()
                    Ok(_) => {} // C (3.c.) Ok()
                    Err(err) => {
                        // C (3.c) Err()
                        show_ok_notification(
                            // B (2.a.) show_ok_notification()
                            app.clone(), // B (2.a.) app.clone();
                            format!("Failed to update Blender repo paths: {:?}", err),
                            tauri_plugin_dialog::MessageDialogKind::Error,
                        );
                        return Err(format!("Failed to update Blender repo paths: {:?}", err));
                        // B (2.b.) priekšlaicīgs return
                    }
                }
            }
        }
        Ok(())
    }
}

/// ID: BV_011
/// ABC analīzes rezultāts:2,6,3
#[tauri::command]
pub async fn fetch_blender_version_installation_locations(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<&str>,
    limit: Option<i64>,
    repo_directory_path: Option<&str>,
) -> Result<Vec<BlenderRepoPath>, String> {
    let repository = BlenderRepoPathRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let results = match repository // A (1.a.) let results =; C (3.b) match
        .fetch(id.as_deref(), limit, repo_directory_path) // B (2.a.) .fetch(); B (2.a.) .as_deref()
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch Blender repo paths: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err));
            // B (2.b.) priekšlaicīgs return
        }
    };
    Ok(results)
}

/// ID: BV_012
/// ABC analīzes rezultāts:7,27,15
#[tauri::command]
pub async fn delete_blender_version_installation_location(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let confirmation = file_system_utility::show_ask_notification(
        // A (1.a.) let confirmation =; B (2.a.) ::show_ask_notification()
        app.clone(), // B (2.a.) app.clone();
        format!("Are you sure you want to delete this Blender installation location?"),
        tauri_plugin_dialog::MessageDialogKind::Warning,
    );
    if confirmation == false {
        // C (3.a.) confirmation == false
        return Ok(()); // B (2.b.) priekšlaicīgs return;
    }
    let blender_repo_path_repository = BlenderRepoPathRepository::new(&state.pool); // A (1.a.) let blender_repo_path_repository =; B (2.a.) ...::new()
    let installed_blender_version_repository = InstalledBlenderVersionRepository::new(&state.pool); // A (1.a.) let installed_blender_version_repository =; B (2.a.) ...::new()
    let mut blender_repo_path_list = match blender_repo_path_repository // A (1.a.) let mut blender_repo_path_list =; C (3.b) match
        .fetch(Some(&id), None, None) // B (2.a.) .fetch()
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch Blender repo paths: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch Blender repo paths: {:?}", err));
            // B (2.b.) priekšlaicīgs return
        }
    };
    if blender_repo_path_list.is_empty() {
        // B (2.a.) .is_empty(); C (3.a) blender_repo_path_list.is_empty() == true
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch Blender repo paths by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch Blender repo paths by ID")); // B (2.b.) priekšlaicīgs return
    }
    let blender_repo_path_entry = blender_repo_path_list.remove(0); // A (1.a.) let blender_repo_path_entry =; B (2.a.) .remove()
    let installed_blender_version_list = match installed_blender_version_repository // A (1.a.) let installed_blender_version_list =; C (3.b) match
        .fetch(None, None, None) // B (2.a.) .fetch()
        .await
    {
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
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
    for version in installed_blender_version_list {
        // A (1.a.) let mut version =;
        if version
            .installation_directory_path
            .starts_with(&blender_repo_path_entry.repo_directory_path)
        {
            // C (3.a) .installation_directory_path.starts_with(&blender_repo_path_entry.repo_directory_path)  == true; B (2.a.) .starts_with()
            match installed_blender_version_repository // C (3.b) match
                .delete(&version.id) // B (2.a.) .delete()
                .await
            {
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => {
                    // C (3.c) Err()
                    show_ok_notification(
                        // B (2.a.) show_ok_notification()
                        app.clone(), // B (2.a.) app.clone();
                        format!(
                            "Failed to delete installed Blender version entry: {:?}",
                            err
                        ),
                        tauri_plugin_dialog::MessageDialogKind::Error,
                    );
                    return Err(format!(
                        "Failed to delete installed Blender version entry: {:?}",
                        err
                    )); // B (2.b.) priekšlaicīgs return
                }
            }
        }
    }
    match blender_repo_path_repository.delete(&id).await {
        // C (3.b) match; B (2.a.) .delete()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete Blender repo path entry: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!(
                "Failed to delete Blender repo path entry: {:?}",
                err
            )); // B (2.b.) priekšlaicīgs return
        }
    }
}
