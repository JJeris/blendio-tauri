use std::io::{Read, Write};

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use zip::ZipArchive;

use crate::AppState;

/// ID: FSU_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:0,12,3
#[tauri::command]
pub async fn instance_popup_window(
    app: AppHandle,
    label: String,
    title: String,
    url_path: String,
) -> Result<(), String> {
    match WebviewWindowBuilder::new(&app, label, WebviewUrl::App(url_path.into())) // C (3.b) match; B (2.a.) ...::new(); B (2.a.) .into()
        .title(title) // B (2.a.) .title()
        .inner_size(600.0, 400.0) // B (2.a.) .inner_size()
        .resizable(true) // B (2.a.) .resizable()
        .always_on_top(true) // B (2.a.) .always_on_top()
        .focused(true) // B (2.a.) .focused()
        .skip_taskbar(true) // B (2.a.) .skip_taskbar()
        .build() // B (2.a.) .build()
    {
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => { // C (3.c) Err()
            show_ok_notification( // B (2.a.) show_ok_notification()
                app.clone(), // B (2.a.) app.clone();
                format!("Failed to instance popup window: {:?}", err),
                tauri_plugin_dialog::MessageDialogKind::Error,
            );
            return Err(format!("Failed to instance popup window: {:?}", err)); // B (2.b.) priekšlaicīgs return
        }
    }
}

/// ID: FSU_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:1,7,3
#[tauri::command]
pub async fn identify_internet_connection() -> Result<bool, String> {
    let result = reqwest::Client::new() // A (1.a.) let result =; B (2.a.) ...::new()
        .get("https://one.one.one.one/") // B (2.a.) .get()
        .timeout(std::time::Duration::from_secs(3)) // B (2.a.) .timeout(); B (2.a.) ::from_secs()
        .send() // B (2.a.) .send()
        .await;
    match result {
        Ok(response) => Ok(response.status().is_success()), // C (3.c.) Ok(); C (3.b) match; B (2.a.) .status(); B (2.a.) .is_success();
        Err(_) => Ok(false), // C (3.c) Err()
    }
}

/// ID: FSU_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:0,5,0
pub fn show_ok_notification(
    app: AppHandle,
    message: String,
    kind: tauri_plugin_dialog::MessageDialogKind,
) -> () {
    app.dialog() // B (2.a.) .dialog()
        .message(message) // B (2.a.) .message()
        .kind(kind) // B (2.a.) .kind()
        .buttons(MessageDialogButtons::Ok) // B (2.a.) .buttons()
        .blocking_show(); // B (2.a.) .blocking_show()
    return ();
}

/// ID: FSU_004
/// Paskaidrojums:
/// ABC analīzes rezultāts:1,7,0
pub fn show_ask_notification(
    app: AppHandle,
    message: String,
    kind: tauri_plugin_dialog::MessageDialogKind,
) -> bool {
    let answer = app // A (1.a.) let answer =;
        .dialog() // B (2.a.) .dialog()
        .message(message) // B (2.a.) .message()
        .kind(kind) // B (2.a.) .kind()
        .buttons(MessageDialogButtons::OkCancelCustom( // B (2.a.) .buttons(
            "Yes".to_string(), // B (2.a.) .to_string()
            "No".to_string(), // B (2.a.) .to_string()
        ))
        .blocking_show(); // B (2.a.) .blocking_show()
    return answer;
}

/// ID: FSU_005
/// Paskaidrojums:
/// ABC analīzes rezultāts:11,26,23
pub async fn extract_archive(
    archive_file_path: std::path::PathBuf,
) -> Result<std::path::PathBuf, String> {
    let file = match std::fs::File::open(&archive_file_path) { // A (1.a.) let file =; C (3.b.) match; B (2.a.) ::open()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to extract archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    };
    let mut archive = match ZipArchive::new(file) { // A (1.a.) let mut archive =; C (3.b.) match; B (2.a.) ...::new()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to extract archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    };
    let archive_dir = match archive_file_path.file_stem() { // A (1.a.) let archive_dir =; C (3.b) match; B (2.a.) .file_stem()
        Some(name) => name.to_string_lossy().to_string(), // C (3.c) Some(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        None => return Err(format!("Failed to extract archive file")), // C (3.c) None =>; B (2.b.) priekšlaicīgs return
    };
    let extract_dir = match archive_file_path.parent() { // A (1.a.) let extract_dir =; C (3.b) match; B (2.a.) .parent()
        Some(parent) => parent, // C (3.c) Some()
        None => return Err(format!("Failed to extract archive file")), // C (3.c) None =>; B (2.b.) priekšlaicīgs return
    };
    for i in 0..archive.len() { // A (1.a.) let i =; B (2.b.) .len()
        let mut inner_file = match archive.by_index(i) { // A (1.a.) let mut inner_file =; C (3.b.) match; B (2.a.) .by_index()
            Ok(val) => val, // C (3.c.) Ok()
            Err(err) => return Err(format!("Failed to extract archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
        };
        let outpath = extract_dir.join(inner_file.name()); // A (1.a.) let outpath =; B (2.a.) .join(); B (2.a.) .name()
        if inner_file.name().ends_with('/') { // C (3.a.) inner_file.name().ends_with('/') == true;  B (2.a.) .name(); B (2.a.) .ends_with()
            match std::fs::create_dir_all(&outpath) { // C (3.b.) match; B (2.a.) ::create_dir_all()
                Ok(_) => {} // C (3.c.) Ok()
                Err(err) => return Err(format!("Failed to extract archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
            }
        } else { // C (3.b.) else
            if let Some(parent) = outpath.parent() { // A (1.d.) if let Some(); B (2.a.) .parent()
                let _ = std::fs::create_dir_all(parent); // A (1.a.) let _ =; B (2.a.) ::create_dir_all()
            }
            let mut outfile = match std::fs::File::create(&outpath) { // A (1.a.) let mut outfile =; C (3.b) match; B (2.a.) ::create()
                Ok(val) => val, // C (3.c.) Ok()
                Err(err) => return Err(format!("Failed to extract archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
            };
            if let Err(err) = std::io::copy(&mut inner_file, &mut outfile) { // A (1.d.) if let Err(); B (2.a.) ::copy()
                return Err(format!("Failed to extract archive file: {:?}", err)); // B (2.b.) priekšlaicīgs return
            }
        }
    }
    Ok(extract_dir.join(archive_dir)) // B (2.a.) .join()
}

/// ID: FSU_006
/// Paskaidrojums:
/// ABC analīzes rezultāts:7,21,21
pub fn archive_file(file_path: std::path::PathBuf) -> Result<std::path::PathBuf, String> {
    let file_name = match file_path.file_name() { // A (1.a.) let file_name =; C (3.b) match; B (2.a.) .file_name()
        Some(val) => val, // C (3.c) Some()
        None => return Err(format!("Failed to archive file")), // C (3.c) None =>; B (2.b.) priekšlaicīgs return
    };
    let zip_path = file_path.with_extension("zip"); // A (1.a.) let zip_path =; B (2.a.) .with_extension()
    let zip_file = match std::fs::File::create(&zip_path) { // A (1.a.) let zip_file =; C (3.b.) match; B (2.a.) ::create()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    };
    let mut zip_writer = zip::ZipWriter::new(zip_file); // A (1.a.) let mut zip_writer =; B (2.a.) ...::new()
    let options: zip::write::FileOptions<()> = // A (1.a.) let options =;
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored); // B (2.a.) ::default(); B (2.a.) .compression_method();
    let mut buffer = Vec::new(); // A (1.a.) let mut buffer =; B (2.a.) ...::new()
    let mut source_file = match std::fs::File::open(&file_path) { // A (1.a.) let mut source_file =; C (3.b) match; B (2.a.) ::open()
        Ok(val) => val, // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    };
    match source_file.read_to_end(&mut buffer) { // C (3.b) match; B (2.a.) .read_to_end()
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
    match zip_writer.start_file(file_name.to_string_lossy().to_string(), options) { // C (3.b) match; B (2.a.) .start_file(); B (2.a.) .to_string_loosy(); B (2.a.) .to_string();
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
    match zip_writer.write_all(&buffer) { // C (3.b) match; B (2.a.) .write_all()
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
    match zip_writer.finish() { // C (3.b) match; B (2.a.) .finish()
        Ok(_) => {} // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to archive file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
    Ok(zip_path)
}

/// ID: FSU_007
/// Paskaidrojums:
/// ABC analīzes rezultāts:3,4,6
pub fn launch_executable(
    executable_file_path: std::path::PathBuf,
    args: Option<Vec<String>>,
) -> Result<(), String> {
    let mut command = std::process::Command::new(executable_file_path); // A (1.a.) let mut command =; B (2.a.) ...::new()
    let arguments = match args { // A (1.a.) let arguments =; C (3.b) match
        Some(val) => val, // C (3.c) Some()
        None => vec![], // C (3.c) None =>; 
    };
    let output = command.args(arguments).output(); // A (1.a.) let output =; B (2.a.) .args(); B (2.a.) .output()
    match output { // C (3.b) match
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to launch executable: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
}

/// ID: FSU_008
/// Paskaidrojums:
/// ABC analīzes rezultāts:1,14,12
pub fn open_in_file_explorer(file_path: std::path::PathBuf) -> Result<(), String> {
    let parent_directory = match file_path.parent() { // A (1.a.) let parent_directory =; C (3.b) match; B (2.a.) .parent()
        Some(val) => val, // C (3.c) Some()
        None => return Err(format!("Failed to open file in file explorer")), // C (3.c) None =>; B (2.b.) priekšlaicīgs return
    };
    #[cfg(target_os = "windows")]
    match std::process::Command::new("explorer") // C (3.b) match; B (2.a.) ...::new()
        .arg(parent_directory) // B (2.a.) .arg()
        .spawn() // B (2.a.) .spawn()
    {
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to open file in file explorer: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
    #[cfg(target_os = "macos")]
    match std::process::Command::new("open") // C (3.b) match; B (2.a.) ...::new()
        .arg(parent_directory) // B (2.a.) .arg()
        .spawn() // B (2.a.) .spawn()
    {
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to open file in file explorer: {:?}", err)), // C (3.c) Err();/ B (2.b.) priekšlaicīgs return
    }
    #[cfg(target_os = "linux")]
    match std::process::Command::new("xdg-open") // C (3.b) match; B (2.a.) ...::new()
        .arg(parent_directory) // B (2.a.) .arg()
        .spawn() // B (2.a.) .spawn()
    {
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to open file in file explorer: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
}

/// ID: FSU_009
/// Paskaidrojums:
/// ABC analīzes rezultāts:3,7,3
pub async fn get_file_from_file_explorer(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Option<std::path::PathBuf>, String> {
    let file_path_option = app // A (1.a.) let file_path_option =; 
        .dialog() // B (2.a.) .dialog()
        .file() // B (2.a.) .file()
        .add_filter("Python Files", &["py"]) // B (2.a.) .add_filter()
        .blocking_pick_file(); // B (2.a.) .blocking_pick_file()
    let file_path_string = match file_path_option { // A (1.a.) let file_path_string =; C (3.b) match
        Some(val) => val.to_string(), // C (3.c) Some(); B (2.a.) .to_string()
        None => return Err(format!("Failed to get file from file explorer")), // C (3.c) None =>; B (2.b.) priekšlaicīgs return
    };
    let file_path = std::path::PathBuf::from(file_path_string); // A (1.a.) let file_path =; B (2.a.) ::from()
    return Ok(Some(file_path));
}

/// ID: FSU_010
/// Paskaidrojums:
/// ABC analīzes rezultāts:3,6,3
pub async fn get_directory_from_file_explorer(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Option<std::path::PathBuf>, String> {
    let directory_path_option = app.dialog().file().blocking_pick_folder(); // A (1.a.) let directory_path_option =; B (2.a.) .dialog(); B (2.a.) .file(); B (2.a.) .blocking_pick_folder()
    let directory_path_string = match directory_path_option { // A (1.a.) let directory_path_string =; C (3.b) match
        Some(val) => val.to_string(), // C (3.c) Some(); B (2.a.) .to_string()
        None => return Err(format!("Failed to get directory from file explorer")), // C (3.c) None =>; B (2.b.) priekšlaicīgs return
    };
    let directory_path = std::path::PathBuf::from(directory_path_string); // A (1.a.) let directory_path =; B (2.a.) ::from()
    return Ok(Some(directory_path));
}

/// ID: FSU_011
/// Paskaidrojums:
/// ABC analīzes rezultāts:0,2,3
pub async fn delete_file(file_path: std::path::PathBuf) -> Result<(), String> {
    match std::fs::remove_file(file_path) { // C (3.b) match; B (2.b.) ::remove_file()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to delete file: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
}

/// ID: FSU_012
/// Paskaidrojums:
/// ABC analīzes rezultāts:0,2,3
pub async fn delete_directory(directory_path: std::path::PathBuf) -> Result<(), String> {
    match std::fs::remove_dir_all(directory_path) { // C (3.b) match; B (2.b.) ::remove_dir_all()
        Ok(_) => Ok(()), // C (3.c.) Ok()
        Err(err) => return Err(format!("Failed to delete directory: {:?}", err)), // C (3.c) Err(); B (2.b.) priekšlaicīgs return
    }
}
