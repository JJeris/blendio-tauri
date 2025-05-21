use crate::{
    db_repo::LaunchArgumentRepository,
    file_system_utility::{self, show_ok_notification},
    models::LaunchArgument,
    AppState,
};
use tauri::AppHandle;

/// ID: KP_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:6,28,10
#[tauri::command]
pub async fn insert_launch_argument(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    argument_string: String,
    project_file_id: Option<String>,
    python_script_id: Option<String>,
) -> Result<String, String> {
    let repository = LaunchArgumentRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository.fetch(None, None, Some(&argument_string)).await {
        // A (1.a.) let mut results =; C (3.b.) match; B (2.a.) repository.fetch();
        Ok(val) => val, // C (3.c) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch launch arguments: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch launch arguments: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    if !results.is_empty() {
        // B (2.a.) results.is_empty(); C (3.a) results.is_empty() != true
        let mut existing_entry = results.remove(0); // A (1.a.) let mut existing_entry =; B (2.a.) results.remove();
        existing_entry.accessed = chrono::Utc::now().to_rfc3339(); // A (1.a.) existing_entry.accessed =; B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        existing_entry.modified = chrono::Utc::now().to_rfc3339(); // A (1.a.) existing_entry.modified =; B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        match repository.update(&existing_entry).await {
            // C (3.b.) match; B (2.a.) repository.update()
            Ok(_) => return Ok(existing_entry.id), // C (3.c) Ok(); B (2.b.) priekšlaicīgs return
            Err(err) => {
                // C (3.c) Err();
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to update existing launch arguments: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to update existing launch arguments: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        }
    }
    let entry = LaunchArgument {
        // A (1.a.)  let entry =;
        id: uuid::Uuid::new_v4().to_string(), // B (2.a.) uuid::Uuid::new_v4(); B (2.a.) .to_string();
        is_default: false,
        argument_string: argument_string,
        last_used_project_file_id: project_file_id,
        last_used_python_script_id: python_script_id,
        created: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        modified: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        accessed: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
    };
    match repository.insert(&entry).await {
        // C (3.b.) match; B (2.a.) repository.insert();
        Ok(_) => Ok(entry.id), // C (3.c) Ok();
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert launch argument: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to insert launch argument: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: KP_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:8,21,17
#[tauri::command]
pub async fn update_launch_argument(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = LaunchArgumentRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository.fetch(Some(&id), None, None).await {
        // A (1.a.) let mut results =; C (3.b.) match; B (2.a.) repository.fetch();
        Ok(val) => val, // C (3.c) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch launch arguments: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch launch arguments: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    if results.is_empty() {
        // B (2.a.) results.is_empty(); C (3.a) results.is_empty() == true
        show_ok_notification(
            // B (2.a.) show_ok_notification()
            app.clone(), // B (2.a.) app.clone();
            format!("Failed to fetch launch arguments by ID"),
            tauri_plugin_dialog::MessageDialogKind::Error,
        );
        return Err(format!("Failed to fetch launch arguments by ID")); // B (2.b.) priekšlaicīgs return
    }
    let mut entry = results.remove(0); // A (1.a.) let mut entry =; B (2.a.) results.remove();
    if is_default == true {
        // C (3.a) is_default == true
        entry.is_default = false; // A (1.a.) entry.is_default =;
        match repository.update(&entry).await {
            // C (3.b.) match; B (2.a.) repository.update()
            Ok(_) => Ok(()), // C (3.c) Ok(); B (2.b.) priekšlaicīgs return
            Err(err) => {
                // C (3.c) Err();
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to update existing launch arguments: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to update existing launch arguments: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        }
    } else { 
        // C (3.b.) else;
        let results = match repository.fetch(None, None, None).await {
            // A (1.a.) let results =; C (3.b.) match; B (2.a.) repository.fetch()
            Ok(val) => val, // C (3.c) Ok();
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
        for mut entry in results {
            // A (1.a.) let mut entry
            let new_default = entry.id == id; // A (1.a.) let new_default =; C (3.a) entry.id == id
            if entry.is_default != new_default {
                // C (3.a) entry.is_default != new_default
                entry.is_default = new_default; // A (1.a.) entry.is_default =;
                match repository.update(&entry).await {
                    // C (3.b.) match;
                    Ok(_) => {} // C (3.c) Ok();
                    Err(err) => {
                        // C (3.c) err();
                        show_ok_notification(
                            // B (2.a.) show_ok_notification()
                            app.clone(), // B (2.a.) app.clone();
                            format!("Failed to update existing launch arguments: {:?}", err),
                            tauri_plugin_dialog::MessageDialogKind::Error,
                        );
                        return Err(format!(
                            "Failed to update existing launch arguments: {:?}",
                            err
                        )); // B (2.b.) priekšlaicīgs return
                    }
                }
            }
        }
        Ok(())
    }
}

/// ID: KP_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:3,8,3
#[tauri::command]
pub async fn fetch_launch_arguments(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    argument_string: Option<String>,
) -> Result<Vec<LaunchArgument>, String> {
    let repository = LaunchArgumentRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository // A (1.a.) let mut results =; // C (3.b.) match
        .fetch(id.as_deref(), limit, argument_string.as_deref()) // B (2.a.) repository.fetch(); B (2.a.) id.as_deref(); B (2.a.) argument_string.as_deref()
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
            return Err(format!("Failed to fetch launch arguments: {:?}", err)); // B(2.b) priekšlaicīgs return
        }
    };
    // Sort DESC
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed)); // A (1.c.) .sort_by(); B (2.a) |a, b| b.accessed.cmp(&a.accessed)
    Ok(results)
}

/// ID: KP_004
/// Paskaidrojums:
/// ABC analīzes rezultāts:2,8,4
#[tauri::command]
pub async fn delete_launch_argument(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let confirmation = file_system_utility::show_ask_notification(
        // A (1.a.) let confirmation =; B (2.a.) show_ask_notification()
        app.clone(), // B (2.a.) app.clone();
        format!("Are you sure you want to delete this launch argument entry?"),
        tauri_plugin_dialog::MessageDialogKind::Warning,
    );
    if confirmation == false {
        // C (3.a.) confirmation == false
        return Ok(()); // B (2.b.) priekšlaicīgs return
    }
    let repository = LaunchArgumentRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    match repository.delete(&id).await {
        // B (2.a.) repository.delete(); // C (3.b) match
        Ok(_) => Ok(()), // C (3.c) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete launch argument: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to delete launch argument: {:?}", err)); // B(2.b) priekšlaicīgs return
        }
    }
}
