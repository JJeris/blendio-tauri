use std::io::{Read, Write};

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use zip::ZipArchive;

use crate::AppState;

/// ID: FSU_001
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn instance_popup_window(
    app: AppHandle,
    label: String,
    title: String,
    url_path: String,
) -> Result<(), String> {
    match WebviewWindowBuilder::new(
        &app,
        label,
        WebviewUrl::App(url_path.into()),
    )
    .title(title)
    .inner_size(600.0, 400.0)
    .resizable(true)
    .always_on_top(true)
    .focused(true)
    .skip_taskbar(true)
    // .position(x, y) // TODO
    .build()
    {
        Ok(_) => Ok(()),
        Err(err) => {
            show_ok_notification(app.clone(), format!("Failed to instance popup window: {:?}", err), tauri_plugin_dialog::MessageDialogKind::Error);
            return Err(format!("Failed to instance popup window: {:?}", err))
        },
    }
}

/// ID: FSU_002
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub async fn identify_internet_connection() -> Result<bool, String> {
    let result = reqwest::Client::new()
        .get("https://one.one.one.one/")
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await;
    match result {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

/// ID: FSU_003
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub fn show_ok_notification(    
    app: AppHandle,
    message: String,
    kind: tauri_plugin_dialog::MessageDialogKind
) -> () {
    app.dialog()
        .message(message)
        .kind(kind)
        .buttons(MessageDialogButtons::Ok)
        .blocking_show();
    return ();
}

/// ID: FSU_004
/// Paskaidrojums:
/// ABC analīzes rezultāts:
#[tauri::command]
pub fn show_ask_notification(    
    app: AppHandle,
    message: String,
    kind: tauri_plugin_dialog::MessageDialogKind
) -> bool {
    let answer = app.dialog()
                        .message(message)
                        .kind(kind)
                        .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
                        .blocking_show();
    return answer;
}

/// ID: FSU_005
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub async fn extract_archive(
    archive_file_path: std::path::PathBuf,
) -> Result<std::path::PathBuf, String> {
    let file = match std::fs::File::open(&archive_file_path) {
        Ok(val) => val,
        Err(err) => {
            return Err(format!("Failed to extract archive file: {:?}", err))
        },
    };
    let mut archive = match ZipArchive::new(file)  {
        Ok(val) => val,
        Err(err) => {
            return Err(format!("Failed to extract archive file: {:?}", err))
        },
    };
    let archive_dir = match archive_file_path.file_stem() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            return Err(format!("Failed to extract archive file"))
        },
    };
    let extract_dir = match archive_file_path.parent() {
        Some(parent) => parent, //.join(archive_dir),
        None => {
            return Err(format!("Failed to extract archive file"))
        },
    };
    for i in 0..archive.len() {
        let mut inner_file = match archive.by_index(i) {
            Ok(val) => val,
            Err(err) => {
                return Err(format!("Failed to extract archive file: {:?}", err))
            },
        };
        let outpath = extract_dir.join(inner_file.name());
        if inner_file.name().ends_with('/') {
            match std::fs::create_dir_all(&outpath) {
                Ok(_) => {}
                Err(err) => {
                    return Err(format!("Failed to extract archive file: {:?}", err))
                },
            }
        } else {
            if let Some(parent) = outpath.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let mut outfile = match std::fs::File::create(&outpath) {
                Ok(val) => val,
                Err(err) => {
                    return Err(format!("Failed to extract archive file: {:?}", err))
                },
            };
            if let Err(err) = std::io::copy(&mut inner_file, &mut outfile) {
                return Err(format!("Failed to extract archive file: {:?}", err))
            }
        }
    }
    Ok(extract_dir.join(archive_dir))
}

/// ID: FSU_006
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub fn archive_file(file_path: std::path::PathBuf) -> Result<std::path::PathBuf, String> {
    let file_name = match file_path.file_name() {
        Some(val) => val,
        None => return Err(format!("Failed to archive file"))
    };
    let zip_path = file_path.with_extension("zip");
    let zip_file = match std::fs::File::create(&zip_path) {
        Ok(val) => val,
        Err(err) => return Err(format!("Failed to archive file: {:?}", err))
    };
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let options: zip::write::FileOptions<()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let mut buffer = Vec::new();
    let mut source_file = match std::fs::File::open(&file_path) {
        Ok(val) => val,
        Err(err) => return Err(format!("Failed to archive file: {:?}", err))
    };
    match source_file.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed to archive file: {:?}", err))
    }
    match zip_writer.start_file(file_name.to_string_lossy().to_string(), options) {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed to archive file: {:?}", err))
    }
    match zip_writer.write_all(&buffer) {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed to archive file: {:?}", err))
    }
    match zip_writer.finish() {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed to archive file: {:?}", err))
    }
    Ok(zip_path)
}

/// ID: FSU_007
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub fn launch_executable(
    executable_file_path: std::path::PathBuf,
    args: Option<Vec<String>>
) -> Result<(), String> {
    let mut command = std::process::Command::new(executable_file_path);
    let arguments = match args {
        Some(val) => val,
        None => vec![],
    };
    let output = command
        .args(arguments)
        .output(); // waits for process to exit
    match output {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(format!("Failed to launch executable: {:?}", err))
        },
    }
}

/// ID: FSU_008
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub fn open_in_file_explorer(file_path: std::path::PathBuf) -> Result<(), String> {
    let parent_directory = match file_path.parent() {
        Some(val) => val,
        None => return Err(format!("Failed to open file in file explorer"))
    };
    #[cfg(target_os = "windows")]
    match std::process::Command::new("explorer").arg(parent_directory).spawn() {
        Ok(_) => Ok(()),
        Err(err) => return Err(format!("Failed to open file in file explorer: {:?}", err))
    }
    #[cfg(target_os = "macos")]
    match std::process::Command::new("open").arg(parent_directory).spawn() {
        Ok(_) => Ok(()),
        Err(err) => return Err(format!("Failed to open file in file explorer: {:?}", err))
    }
    #[cfg(target_os = "linux")]
    match std::process::Command::new("xdg-open").arg(parent_directory).spawn() {
        Ok(_) => Ok(()),
        Err(err) => return Err(format!("Failed to open file in file explorer: {:?}", err))
    }
}

/// ID: FSU_009
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub async fn get_file_from_file_explorer(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Option<std::path::PathBuf>, String> {
    let file_path_option = app.dialog().file().add_filter("Python Files", &["py"]).blocking_pick_file();
    let file_path_string = match file_path_option {
        Some(val) => val.to_string(),
        None => return Err(format!("Failed to get file from file explorer"))
    };
    let file_path = std::path::PathBuf::from(file_path_string);
    return Ok(Some(file_path));
}

/// ID: FSU_010
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub async fn get_directory_from_file_explorer(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Option<std::path::PathBuf>, String> {
    let directory_path_option = app.dialog().file().blocking_pick_folder();
    let directory_path_string = match directory_path_option {
        Some(val) => val.to_string(),
        None => return Err(format!("Failed to get directory from file explorer"))
    };
    let directory_path = std::path::PathBuf::from(directory_path_string);
    return Ok(Some(directory_path));
}

/// ID: FSU_011
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub async fn delete_file(
    file_path: std::path::PathBuf,
) -> Result<(), String> {
    match std::fs::remove_file(file_path) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(format!("Failed to delete file: {:?}", err))
        },
    }
}

/// ID: FSU_012
/// Paskaidrojums:
/// ABC analīzes rezultāts:
pub async fn delete_directory(
    directory_path: std::path::PathBuf,
) -> Result<(), String> {
    match std::fs::remove_dir_all(directory_path) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(format!("Failed to delete directory: {:?}", err))
        },
    }
}