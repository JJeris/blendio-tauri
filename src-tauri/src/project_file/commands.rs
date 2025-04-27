//! Contains the frontend exposed commands of the Project file module.

/// Saglabāt .blend failu datus
#[tauri::command]
pub async fn insert_blend_file_data() -> Result<(), String> {
    Ok(())
}

/// Atvērt .blend failu Blender versijā
#[tauri::command]
pub async fn open_blend_file(blend_file_path: Option<std::path::PathBuf>) -> Result<(), String> {
    Ok(())
}

/// Izveidot jaunu .blend failu
#[tauri::command]
pub async fn create_blend_file() -> Result<(), String> {
    Ok(())
}

/// Izdzēst .blend failu
#[tauri::command]
pub async fn delete_blend_file() -> Result<(), String> {
    Ok(())
}

/// Atrast .blend failu lokālajā failu sistēmā
#[tauri::command]
pub async fn find_blend_file_in_local_file_system() -> Result<(), String> {
    Ok(())
}

/// Saglabāt .blend failus arhīvfailā
#[tauri::command]
pub async fn create_blend_file_archive() -> Result<(), String> {
    Ok(())
}
