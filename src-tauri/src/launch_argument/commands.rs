use chrono::Utc;
use tauri::AppHandle;
use uuid::Uuid;
use crate::{db_repo::LaunchArgumentRepository, file_system_utility::{self, show_ok_notification}, models::LaunchArgument, AppState};

/// ID: KP_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn insert_launch_argument(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    argument_string: String,
    project_file_id: Option<String>,
    python_script_id: Option<String>
) -> Result<String, String> {
    let repository = LaunchArgumentRepository::new(&state.pool);
    let mut results = match repository.fetch(None, None, Some(&argument_string)).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch launch arguments: {:?}", err))
        },
    };
    if !results.is_empty() {
        let mut existing_entry = results.remove(0);
        existing_entry.accessed = chrono::Utc::now().to_rfc3339();
        existing_entry.modified = chrono::Utc::now().to_rfc3339();
        match repository.update(&existing_entry).await {
            Ok(_) => return Ok(existing_entry.id),
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to update existing launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to update existing launch arguments: {:?}", err))
            },
        }
    }
    let entry = LaunchArgument{ 
        id: uuid::Uuid::new_v4().to_string(), 
        is_default: false, 
        argument_string: argument_string, 
        last_used_project_file_id: project_file_id, 
        last_used_python_script_id: python_script_id, 
        created: chrono::Utc::now().to_rfc3339(),
        modified: chrono::Utc::now().to_rfc3339(),
        accessed: chrono::Utc::now().to_rfc3339(), 
    };
    match repository.insert(&entry).await {
        Ok(_) => Ok(entry.id),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to insert launch argument: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert launch argument: {:?}", err))
        },
    }
}

/// ID: KP_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn update_launch_argument(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = LaunchArgumentRepository::new(&state.pool);
    let mut results = match repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch launch arguments: {:?}", err))
        },
    };
    if results.is_empty() {
        show_ok_notification(app.clone(), format!("Failed to fetch launch arguments by ID"), tauri_plugin_dialog::MessageDialogKind::Error);
        return Err(format!("Failed to fetch launch arguments by ID"))
    }
    let mut entry = results.remove(0);
    if is_default == true 
    {
        entry.is_default = false;
        match repository.update(&entry).await {
            Ok(_) => Ok(()),
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to update existing launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to update existing launch arguments: {:?}", err))
            },
        }
    }
    else
    {
        let results = match repository.fetch(None, None, None).await {
            Ok(val) => val,
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to fetch launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to fetch launch arguments: {:?}", err))
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
                        show_ok_notification(app.clone(), format!("Failed to update existing launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                        return Err(format!("Failed to update existing launch arguments: {:?}", err))
                    },
                }
            }
        }
        Ok(())
    }
}

/// ID: KP_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn fetch_launch_arguments(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    argument_string: Option<String>
) -> Result<Vec<LaunchArgument>, String> {
    let repository = LaunchArgumentRepository::new(&state.pool);
    let mut results = match repository.fetch(id.as_deref(), limit, argument_string.as_deref()).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch launch arguments: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch launch arguments: {:?}", err))
        },
    };
    // Sort DESC
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed));
    Ok(results)
}

/// ID: KP_004
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn delete_launch_argument(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let confirmation= file_system_utility::show_ask_notification(app.clone(), format!("Are you sure you want to delete this launch argument entry?"), tauri_plugin_dialog::MessageDialogKind::Warning);
    if confirmation == false {
        return Ok(());
    }
    let repository = LaunchArgumentRepository::new(&state.pool);
    match repository.delete(&id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to delete launch argument: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to delete launch argument: {:?}", err))
        },
    }
}