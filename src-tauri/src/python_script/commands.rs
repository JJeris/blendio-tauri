use tauri::AppHandle;
use crate::{db_repo::PythonScriptRepository, file_system_utility::{self, show_ok_notification}, models::{PythonScript}, AppState};

/// ID: PS_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn insert_python_script(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Option<PythonScript>, String> {
    let file_path_option = match file_system_utility::get_file_from_file_explorer(app.clone(), state.clone()).await {
        Ok(val) => val,
        Err(_) => return Ok(None)
    };
    let file_path = match file_path_option {
        Some(val) => val,
        None => {
            return Ok(None)
        },
    };
    let repository = PythonScriptRepository::new(&state.pool);
    let mut results = match repository.fetch(None, None, file_path.to_str()).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch python scripts: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch python scripts: {:?}", err))
        },
    };
    if !results.is_empty() {
        let mut existing_entry = results.remove(0);
        existing_entry.accessed = chrono::Utc::now().to_rfc3339();
        existing_entry.modified = chrono::Utc::now().to_rfc3339();
        match repository.update(&existing_entry).await {
            Ok(_) => return Ok(Some(existing_entry)),
            Err(err) => {
                show_ok_notification(app.clone(), format!("Failed to update existing python scripts: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
                return Err(format!("Failed to update existing python scripts: {:?}", err))
            },
        }
    }
    // If not, create and insert new entry
    let entry = PythonScript {
        id: uuid::Uuid::new_v4().to_string(),
        script_file_path: file_path.to_string_lossy().to_string(),
        created: chrono::Utc::now().to_rfc3339(),
        modified: chrono::Utc::now().to_rfc3339(),
        accessed: chrono::Utc::now().to_rfc3339(),
    };
    match repository.insert(&entry).await {
        Ok(_) => Ok(Some(entry)),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to insert python script: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to insert python script: {:?}", err))
        },
    }
}

/// ID: PS_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn fetch_python_scripts(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    script_file_path: Option<String>
) -> Result<Vec<PythonScript>, String> {
    let repository = PythonScriptRepository::new(&state.pool);
    let mut results = match repository.fetch(id.as_deref(), limit, script_file_path.as_deref()).await {
        Ok(val) => val,
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to fetch python scripts: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to fetch python scripts: {:?}", err))
        },
    };
    // Sort DESC
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed));
    Ok(results)
}

/// ID: PS_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn delete_python_script(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let confirmation= file_system_utility::show_ask_notification(app.clone(), format!("Are you sure you want to delete this python script entry?"), tauri_plugin_dialog::MessageDialogKind::Warning);
    if confirmation == false {
        return Ok(());
    }
    let repository = PythonScriptRepository::new(&state.pool);
    match repository.delete(&id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to delete python script: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to delete python script: {:?}", err))
        },
    }
}