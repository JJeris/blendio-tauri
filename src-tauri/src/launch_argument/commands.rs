use chrono::Utc;
use uuid::Uuid;

use crate::{db_repo::LaunchArgumentRepository, models::LaunchArgument, AppState};


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
        Err(err) => return Err(String::new()),
    };

    if !results.is_empty() {
        let mut existing_entry = results.remove(0);
        existing_entry.accessed = Utc::now().to_rfc3339();
        existing_entry.modified = Utc::now().to_rfc3339();
        match repository.update(&existing_entry).await {
            Ok(_) => return Ok(existing_entry.id),
            Err(err) => return Err(String::new()),
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
        Err(err) => return Err(String::new()),
    }
    Ok(entry.id)
}

#[tauri::command]
pub async fn update_launch_argument(
    state: tauri::State<'_, AppState>,
    id: String,
    is_default: bool,
) -> Result<(), String> {
    let repository = LaunchArgumentRepository::new(&state.pool);
    let mut results = match repository.fetch(Some(&id), None, None).await {
        Ok(val) => val,
        Err(err) => return Err(String::new()),
    };
    if results.is_empty() {
        return Err(String::new())
    }
    let mut entry = results.remove(0);
    if is_default == true 
    {
        entry.is_default = false;
        match repository.update(&entry).await {
            Ok(_) => Ok(()),
            Err(err) => return Err(String::new()),
        }
    }
    else
    {
        let results = match repository.fetch(None, None, None).await {
            Ok(val) => val,
            Err(err) => return Err(String::new()),
        };
        for mut entry in results {
            let new_default = entry.id == id; // TODO fix.
            if entry.is_default != new_default
            {
                entry.is_default = new_default;
                match repository.update(&entry).await {
                    Ok(_) => {},
                    Err(err) => return Err(String::new()),
                }
            }
        }
        Ok(())
    }
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
        Err(err) => return Err(String::new()),
    };
    results.sort_by(|a, b| b.accessed.cmp(&a.accessed));
    Ok(results)
}

#[tauri::command]
pub async fn delete_launch_argument(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let repository = LaunchArgumentRepository::new(&state.pool);
    match repository.delete(&id).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new()),
    }
}