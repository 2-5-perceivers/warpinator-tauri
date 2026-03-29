use std::path::PathBuf;

use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
use warpinator_lib::remote_manager::RemoteManager;
use warpinator_lib::types::transfer::Transfer;

#[tauri::command]
pub async fn new_transfer(
    app_handle: tauri::AppHandle,
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
    send_folders: bool,
) -> Result<(), String> {
    let remote = remotes.get_worker(remote_uuid.as_str()).await.ok_or("No remote found")?;
    let dialog_builder =
        app_handle.dialog().file().set_parent(&app_handle.get_webview_window("main").unwrap());
    let files = if send_folders {
        dialog_builder
            .set_title("Select folders to send")
            .blocking_pick_folders()
            .ok_or("No folders selected")?
    } else {
        dialog_builder
            .set_title("Select files to send")
            .blocking_pick_files()
            .ok_or("No files selected")?
    };
    let paths = files
        .into_iter()
        .map(|path| path.into_path())
        .collect::<Result<Vec<PathBuf>, _>>()
        .map_err(|_| "Failed to convert file paths")?;
    remote.send_transfer_request(paths).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_transfers(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
) -> Result<Vec<Transfer>, String> {
    Ok(remotes.transfers(remote_uuid.as_str()).await.unwrap_or_default())
}
