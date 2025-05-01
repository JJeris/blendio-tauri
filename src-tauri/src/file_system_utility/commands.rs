use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_download_popup(app: AppHandle) -> Result<(), String> {
    match WebviewWindowBuilder::new(
        &app,
        "download-popup",
        WebviewUrl::App("popup".into()),
    )
    .title("Choose Download Location")
    .inner_size(500.0, 400.0)
    .resizable(false)
    .always_on_top(true)
    .focused(true)
    .skip_taskbar(true)
    .build()
    {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to create popup window.".into()),
    }
}
