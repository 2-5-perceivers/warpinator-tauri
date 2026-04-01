use std::path::{Path, PathBuf};

use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;
use warpinator_lib::remote_manager::RemoteManager;
use warpinator_lib::types::transfer::Transfer;
use warpinator_lib::types::transfer::TransferKind::Incoming;

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

#[tauri::command]
pub async fn accept_transfer(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
    transfer_uuid: String,
    destination: &Path,
) -> Result<(), String> {
    tokio::fs::create_dir_all(destination).await.map_err(|e| e.to_string())?;
    let remote = remotes.get_worker(remote_uuid.as_str()).await.ok_or("No remote found")?;
    remote.accept_transfer(transfer_uuid.as_str(), destination).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cancel_transfer(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
    transfer_uuid: String,
) -> Result<(), String> {
    let remote = remotes.get_worker(remote_uuid.as_str()).await.ok_or("No remote found")?;
    remote.cancel_transfer(transfer_uuid.as_str()).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_transfer(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
    transfer_uuid: String,
) -> Result<(), String> {
    let remote = remotes.get_worker(remote_uuid.as_str()).await.ok_or("No remote found")?;
    remote.stop_transfer(transfer_uuid.as_str(), false).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_transfer(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
    transfer_uuid: String,
) -> Result<(), String> {
    remotes
        .remove_transfer(remote_uuid.as_str(), transfer_uuid.as_str())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_transfer(
    app_handle: tauri::AppHandle,
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
    transfer_uuid: String,
) -> Result<(), String> {
    let transfer = remotes
        .transfer(remote_uuid.as_str(), transfer_uuid.as_str())
        .await
        .ok_or("No transfer found")?;
    if let Incoming { destination, .. } = transfer.kind {
        if transfer.entry_names.len() > 1 {
            let paths_iter = transfer.entry_names.iter().map(|name| destination.join(name));
            app_handle.opener().reveal_items_in_dir(paths_iter).map_err(|e| e.to_string())?;
        } else {
            let path = destination.join(transfer.entry_names[0].as_str());
            let metadata = tokio::fs::metadata(&path).await.map_err(|e| e.to_string())?;
            if metadata.is_dir() {
                app_handle.opener().reveal_item_in_dir(path).map_err(|e| e.to_string())?;
            } else {
                app_handle
                    .opener()
                    .open_path(path.to_string_lossy(), None::<&str>)
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    } else {
        Err("Can't open an outgoing transfer".to_string())
    }
}
