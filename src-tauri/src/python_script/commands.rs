//! Contains the frontend exposed commands of the Project file module.

use chrono::Utc;
use tauri::AppHandle;
use uuid::Uuid;

use crate::{db_repo::PythonScriptRepository, file_system_utility, models::{ProjectFile, PythonScript}, AppState};

/// Saglabāt nesen izmantoto Python skripta faila lokāciju
#[tauri::command]
pub async fn insert_python_script(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<PythonScript, String> {
    let file_path = match file_system_utility::get_file_from_file_explorer(app, state.clone()).await {
        Ok(val) => val,
        Err(_) => return Err(String::new()),
    };
    let file_path_str = file_path.to_string_lossy().to_string();
    let repository = PythonScriptRepository::new(&state.pool);
    let mut results = match repository.fetch(None, None, Some(file_path_str.clone())).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if !results.is_empty() {
        let mut existing_entry = results.remove(0);
        existing_entry.accessed = Utc::now().to_rfc3339();
        existing_entry.modified = Utc::now().to_rfc3339();
        match repository.update(&existing_entry).await {
            Ok(_) => return Ok(existing_entry),
            Err(_) => return Err(String::new()),
        }
    }
    // If not, create and insert new entry
    let entry = PythonScript {
        id: Uuid::new_v4().to_string(),
        script_file_path: file_path_str,
        created: Utc::now().to_rfc3339(),
        modified: Utc::now().to_rfc3339(),
        accessed: Utc::now().to_rfc3339(),
    };
    match repository.insert(&entry).await {
        Ok(_) => Ok(entry),
        Err(_) => Err(String::new()),
    }
}


#[tauri::command]
pub async fn fetch_python_scripts(
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    script_file_path: Option<String>
) -> Result<Vec<PythonScript>, String> {
    let repository = PythonScriptRepository::new(&state.pool);
    let mut results = match repository.fetch(id.as_deref(), limit, script_file_path).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };

    results.sort_by(|a, b| b.accessed.cmp(&a.accessed)); // DESC
    Ok(results)
}

#[tauri::command]
pub async fn delete_python_script(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let repository = PythonScriptRepository::new(&state.pool);
    match repository.delete(&id).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}