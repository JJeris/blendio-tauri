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
OK- Blender installation
OK- Blender uninstallation
OK- Blender launch (basic).
OK- Project files
OK- Reveal, 
OK- archive, 
OK- create, 
OK- open.
OK?- Read in existing Blender versions (?)
- Launch arguments
- Python scripts
- Remove zustand, not needed.

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


    // let mut command = std::process::Command::new(executable_file_path);
    // let arguments = match args {
    //     Some(val) => val,
    //     None => vec![],
    // };
    // command
    //     .stdout(std::process::Stdio::null())
    //     .stderr(std::process::Stdio::null());
    // match command.spawn() {
    //     Ok(_) => Ok(()),
    //     Err(e) => Err(format!("Failed to launch executable: {}", e)),
    // }

    pub const SAVE_BLEND_FILE_PYTHON_EXPRESSION: &str = 
r#"
import bpy
bpy.ops.wm.save_as_mainfile(filepath="C:\\test\\test.blend")
"#;
