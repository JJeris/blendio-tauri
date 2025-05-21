use crate::{
    db_repo::PythonScriptRepository,
    file_system_utility::{self, show_ok_notification},
    models::PythonScript,
    AppState,
};
use tauri::AppHandle;

/// ID: PS_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:8,36,16
#[tauri::command]
pub async fn insert_python_script(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Option<PythonScript>, String> {
    let file_path_option = // A (1.a.) let file_path_option =;
        match file_system_utility::get_file_from_file_explorer(app.clone(), state.clone()).await { // C (3.b.) match; B (2.a.) get_file_from_file_explorer(); // B (2.a.) app.clone(); B (2.a.) state.clone()
            Ok(val) => val, // C (3.c) Ok()
            Err(_) => return Ok(None), // C (3.c) Err(); // B (2.b.) priekšlaicīgs return
        };
    let file_path = match file_path_option {
        // A (1.a.) let file_path; C (3.b.) match;
        Some(val) => val,        // C (3.c) Some()
        None => return Ok(None), // C (3.c) None =>; B (2.b.) priekšlaicīgs return
    };
    let repository = PythonScriptRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository.fetch(None, None, file_path.to_str()).await {
        // A (1.a.) let mut results =; C (3.b.) match; B (2.a.) repository.fetch(); B (2.a.) file_path.to_str();
        Ok(val) => val, // C (3.c) Ok()
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to fetch python scripts: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to fetch python scripts: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    };
    if !results.is_empty() {
        // B (2.a.) results.is_empty(); C (3.a) results.is_empty() != true
        let mut existing_entry = results.remove(0); // A (1.a.) let mut existing_entry =; B (2.a.) results.remove();
        existing_entry.accessed = chrono::Utc::now().to_rfc3339(); // A (1.a.) existing_entry.accessed =; B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        existing_entry.modified = chrono::Utc::now().to_rfc3339(); // A (1.a.) existing_entry.modified =; B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        match repository.update(&existing_entry).await {
            // C (3.b.) match; B (2.a.) repository.update()
            Ok(_) => return Ok(Some(existing_entry)), // C (3.c) Ok(); B (2.b.) priekšlaicīgs return
            Err(err) => {
                // C (3.c) Err();
                show_ok_notification(
                    // B (2.a.) show_ok_notification()
                    app.clone(), // B (2.a.) app.clone();
                    format!("Failed to update existing python scripts: {:?}", err),
                    tauri_plugin_dialog::MessageDialogKind::Error,
                );
                return Err(format!(
                    "Failed to update existing python scripts: {:?}",
                    err
                )); // B (2.b.) priekšlaicīgs return
            }
        }
    }
    // If not, create and insert new entry
    let entry = PythonScript {
        // A (1.a.)  let entry =;
        id: uuid::Uuid::new_v4().to_string(), // B (2.a.) uuid::Uuid::new_v4(); B (2.a.) .to_string();
        script_file_path: file_path.to_string_lossy().to_string(), // B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        created: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        modified: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
        accessed: chrono::Utc::now().to_rfc3339(), // B (2.a.) chrono::Utc::now(); B (2.a.) .to_rfc3339();
    };
    match repository.insert(&entry).await {
        // C (3.b.) match; B (2.a.) repository.insert();
        Ok(_) => Ok(Some(entry)), // C (3.c) Ok();
        Err(err) => {
            // C (3.c) Err();
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to insert python script: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to insert python script: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: PS_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:3,8,3
#[tauri::command]
pub async fn fetch_python_scripts(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
    limit: Option<i64>,
    script_file_path: Option<String>,
) -> Result<Vec<PythonScript>, String> {
    let repository = PythonScriptRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    let mut results = match repository // A (1.a.) let mut results =; // C (3.b.) match
        .fetch(id.as_deref(), limit, script_file_path.as_deref()) // B (2.a.) repository.fetch(); B (2.a.) id.as_deref(); B (2.a.) script_file_path.as_deref()
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
            return Err(format!("Failed to fetch python scripts: {:?}", err)); // B(2.b) priekšlaicīgs return
        }
    };
    // Sort DESC
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed)); // A (1.c.) sort_by(); B (2.a) |a, b| b.accessed.cmp(&a.accessed)
    Ok(results)
}

/// ID: PS_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:2,8,4
#[tauri::command]
pub async fn delete_python_script(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let confirmation = file_system_utility::show_ask_notification(
        // A (1.a.) let confirmation =; B (2.a.) show_ask_notification()
        app.clone(), // B (2.a.) app.clone();
        format!("Are you sure you want to delete this python script entry?"),
        tauri_plugin_dialog::MessageDialogKind::Warning,
    );
    if confirmation == false {
        // C (3.a.) confirmation == false
        return Ok(()); // B (2.b.) priekšlaicīgs return
    }
    let repository = PythonScriptRepository::new(&state.pool); // A (1.a.) let repository =; B (2.a.) ...::new()
    match repository.delete(&id).await {
        // B (2.a.) repository.delete(); // C (3.b) match
        Ok(_) => Ok(()), // C (3.c) Ok()
        Err(err) => {
            // C (3.c) Err()
            show_ok_notification(
                // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to delete python script: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to delete python script: {:?}", err)); // B(2.b) priekšlaicīgs return
        }
    }
}
