use std::io::{Read, Write};

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::DialogExt;
use zip::ZipArchive;

use crate::AppState;

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
        Err(_) => Err("Failed to create popup window.".into()),
    }
}

pub async fn extract_archive(
    archive_file_path: std::path::PathBuf,
) -> Result<std::path::PathBuf, String> {
    let file = match std::fs::File::open(&archive_file_path) {
        Ok(val) => val,
        Err(err) => return Err(String::new())
    };
    let mut archive = match ZipArchive::new(file)  {
        Ok(val) => val,
        Err(err) => return Err(String::new())
    };
    let archive_dir = match archive_file_path.file_stem() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err("Invalid archive file name".into()),
    };
    let extract_dir = match archive_file_path.parent() {
        Some(parent) => parent, //.join(archive_dir),
        None => return Err(String::new())
    };
    for i in 0..archive.len() {
        let mut inner_file = match archive.by_index(i) {
            Ok(val) => val,
            Err(_) => return Err(String::new())
        };
        let outpath = extract_dir.join(inner_file.name());
        if inner_file.name().ends_with('/') {
            match std::fs::create_dir_all(&outpath) {
                Ok(_) => {}
                Err(_) => return Err("Failed to create directory".into()),
            }
        } else {
            if let Some(parent) = outpath.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let mut outfile = match std::fs::File::create(&outpath) {
                Ok(val) => val,
                Err(_) => return Err("Failed to create output file".into()),
            };
            if let Err(_) = std::io::copy(&mut inner_file, &mut outfile) {
                return Err("Failed to copy file contents".into());
            }
        }
    }
    Ok(extract_dir.join(archive_dir))
}

pub async fn delete_file(
    file_path: std::path::PathBuf,
) -> Result<(), String> {
    match std::fs::remove_file(file_path) {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new())
    }
}

pub async fn delete_directory(
    directory_path: std::path::PathBuf,
) -> Result<(), String> {
    match std::fs::remove_dir_all(directory_path) {
        Ok(_) => Ok(()),
        Err(err) => return Err(String::new())
    }
}

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
        // .stdout(std::process::Stdio::null())
        // .stderr(std::process::Stdio::null())
        .output(); // waits for process to exit
    match output {
        Ok(_) => Ok(()),
        Err(_) => Err(String::new()),
    }
}

pub fn open_in_file_explorer(file_path: std::path::PathBuf) -> Result<(), String> {
    let parent_directory = match file_path.parent() {
        Some(val) => val,
        None => return Err(String::new()),
    };
    #[cfg(target_os = "windows")]
    match std::process::Command::new("explorer").arg(parent_directory).spawn() {
        Ok(_) => Ok(()),
        Err(e) => return Err(String::new()),
    }
    #[cfg(target_os = "macos")]
    match std::process::Command::new("open").arg(parent_directory).spawn() {
        Ok(_) => Ok(()),
        Err(e) => return Err(String::new()),
    }
    #[cfg(target_os = "linux")]
    match std::process::Command::new("xdg-open").arg(parent_directory).spawn() {
        Ok(_) => Ok(()),
        Err(e) => return Err(String::new()),
    }
}

pub async fn get_file_from_file_explorer(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<std::path::PathBuf, String> {
    let directory_path = app.dialog().file().blocking_pick_file();
    let file_path_string = match directory_path {
        Some(val) => val.to_string(),
        None => return Err(String::new()),
    };
    let file_path = std::path::PathBuf::from(file_path_string);
    return Ok(file_path);
}

pub fn archive_file(file_path: std::path::PathBuf) -> Result<std::path::PathBuf, String> {
    let file_name = match file_path.file_name() {
        Some(val) => val,
        None => return Err(String::new()),
    };
    let zip_path = file_path.with_extension("zip");
    let zip_file = match std::fs::File::create(&zip_path) {
        Ok(val) => val,
        Err(e) => return Err(String::new()),
    };
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let options: zip::write::FileOptions<()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let mut buffer = Vec::new();
    let mut source_file = match std::fs::File::open(&file_path) {
        Ok(val) => val,
        Err(e) => return Err(String::new()),
    };
    match source_file.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(e) => return Err(String::new()),
    }
    match zip_writer.start_file(file_name.to_string_lossy().to_string(), options) {
        Ok(_) => {},
        Err(e) => return Err(String::new()),
    }
    match zip_writer.write_all(&buffer) {
        Ok(_) => {},
        Err(e) => return Err(String::new()),
    }
    match zip_writer.finish() {
        Ok(_) => {},
        Err(e) => return Err(String::new()),
    }
    Ok(zip_path)
}