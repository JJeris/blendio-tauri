//! Contains the frontend exposed commands of the Project file module.

/// Saglabāt nesen izmantoto Python skripta faila lokāciju
#[tauri::command]
pub async fn insert_recently_used_python_script_file_paths() -> Result<(), String> {
    Ok(())
}

/// Atrast Python skripta failu lokālajā failu sistēmā
#[tauri::command]
pub async fn find_python_script_file_in_local_file_system(
    file_path: Option<std::path::PathBuf>,
) -> Result<(), String> {
    Ok(())
}
