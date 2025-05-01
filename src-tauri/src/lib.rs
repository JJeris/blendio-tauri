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
use crate::project_file::*;
use crate::python_script::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .setup(|app| {
            // Set up database manually
            tauri::async_runtime::spawn(async move {
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
            });
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
            insert_installed_blender_version_data,
            run_blender_version,
            get_downloadable_blender_version_data,
            download_and_install_blender_version,
            uninstall_blender_version,
            insert_blender_version_installation_location,
            //
            insert_blend_file_data,
            open_blend_file,
            create_blend_file,
            delete_blend_file,
            find_blend_file_in_local_file_system,
            create_blend_file_archive,
            //
            insert_recently_used_python_script_file_paths,
            find_python_script_file_in_local_file_system,
            //
            // insert_user,
            // get_users,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
