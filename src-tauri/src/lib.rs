use tauri::Manager;

mod db_context;
mod db_repo;
mod models;

use crate::db_repo::*;
use crate::models::*;

mod blender_version;
mod file_system_utility;
mod project_file;
mod python_script;

use crate::blender_version::*;
use crate::file_system_utility::*;
use crate::project_file::*;
use crate::python_script::*;

#[derive(Debug)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let mut base_dir = dirs::data_dir().expect("Failed to get data dir");
    base_dir.push("com.bakalaurs.blendio-tauri");
    std::fs::create_dir_all(&base_dir).expect("Failed to create app data directory");

    base_dir.push("test.db");
    if !base_dir.exists() {
        std::fs::File::create(&base_dir).expect("Failed to create database file");
    }

    let db_url = format!("sqlite://{}", base_dir.to_string_lossy());

    let pool = sqlx::SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let app_state = AppState { pool };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .plugin(tauri_plugin_upload::init())
        .setup(|app| {
            // Opens the developer tools when run in debug.
            let window = app.get_webview_window("main").unwrap();
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            //
            insert_installed_blender_version,
            update_installed_blender_version,
            fetch_installed_blender_versions,
            uninstall_and_delete_installed_blender_version_data,
            launch_blender_version_with_launch_args,
            get_downloadable_blender_version_data,
            download_and_install_blender_version,
            insert_blender_version_installation_location,
            //
            insert_blend_file,
            insert_and_refresh_blend_files,
            fetch_blend_files,
            open_blend_file,
            create_new_project_file,
            delete_blend_file,
            reveal_project_file_in_local_file_system,
            create_project_file_archive_file,
            insert_launch_argument,
            fetch_launch_arguments,
            //
            insert_python_script,
            fetch_python_scripts,
            find_python_script_file_in_local_file_system,
            //
            insert_blender_version_installation_location,
            update_blender_version_installation_location,
            fetch_blender_version_installation_locations,
            delete_blender_version_installation_location,
            //
            instance_popup_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
