cs src-tauri
cargo install sqlx-cli --no-default-features --features sqlite
sqlx database create --database-url sqlite:///C:/Users/J/AppData/Roaming/com.bakalaurs.blendio-tauri/test.db
sqlx migrate add create_users_table
sqlx migrate run --database-url sqlite:///C:/Users/J/AppData/Roaming/com.bakalaurs.blendio-tauri/test.db
sqlx migrate run --database-url sqlite:///C:\Users\J\AppData\Roaming\com.bakalaurs.blendio-tauri\test.db

TODO:
OK- db set up models and migratios
OK- Blender installation locations
OK- Blender download
- Blender installation
- Project files
- Launch arguments (keep only a maxmimum of 20, overwrite older)
- Python projects
- Do electronJS equivalent.

- Finish projektējums
- Code analysis.
- Performance analysis
- Results
- Theory.


// pub fn get_database_url(app: &AppHandle) -> Result<String, String> {
//     let base_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
//     let db_path = base_dir.join("test.db");
//     Ok(format!("sqlite://{}", db_path.to_string_lossy()))
// }