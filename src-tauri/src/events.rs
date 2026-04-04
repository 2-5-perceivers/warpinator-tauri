use std::error::Error;
use std::path::PathBuf;

use tauri::Wry;
use tauri::path::PathResolver;
use tauri_plugin_notification::Notification;
use tauri_plugin_store::Store;
use warpinator_lib::remote_manager::{RemoteManager, WarpEvent};

fn resolve_destination(store: &Store<Wry>, path: &PathResolver<Wry>) -> Result<PathBuf, String> {
    store
        .get("default-destination")
        .map_or_else(
            || path.download_dir().map(|v| v.join("Warpinator")).ok(),
            |v| v.as_str().map(Into::into),
        )
        .ok_or_else(|| "No default destination set".into())
}

pub trait HandleEvent {
    /// Checks if an event is auto acceptable and then tries to auto-accept.
    /// Returns true if so
    async fn try_auto_accept(
        &self,
        store: &Store<Wry>,
        path: &PathResolver<Wry>,
        remote_manager: &RemoteManager,
    ) -> Result<bool, String>;

    ///  Generates notifications for the event
    async fn try_notify(
        &self,
        store: &Store<Wry>,
        notification_ext: &Notification<Wry>,
        remote_manager: &RemoteManager,
    ) -> Result<(), String>;
}

impl HandleEvent for WarpEvent {
    async fn try_auto_accept(
        &self,
        store: &Store<Wry>,
        path: &PathResolver<Wry>,
        remote_manager: &RemoteManager,
    ) -> Result<bool, String> {
        if let WarpEvent::TransferAdded(remote_uuid, transfer_uuid) = self {
            match store
                .get("auto-accept")
                .map(|v| v.as_str().unwrap_or("none").to_string())
                .unwrap_or("none".to_string())
                .as_str()
            {
                "all" => {
                    let destination = resolve_destination(store, path)?;
                    remote_manager
                        .get_worker(remote_uuid)
                        .await
                        .ok_or("Remote not found")?
                        .accept_transfer(transfer_uuid, destination)
                        .await
                        .map_err(|_| "Failed to accept transfer")?;
                    return Ok(true);
                }
                "favorites" => {
                    let favorites: Vec<String> = store
                        .get("favorites")
                        .and_then(|v| v.as_array().cloned()) // None if missing or not an array
                        .unwrap_or_default() // empty vec, not an error
                        .iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect();

                    if favorites.contains(remote_uuid) {
                        let destination = resolve_destination(store, path)?;
                        remote_manager
                            .get_worker(remote_uuid)
                            .await
                            .ok_or("Remote not found")?
                            .accept_transfer(transfer_uuid, destination)
                            .await
                            .map_err(|_| "Failed to accept transfer")?;
                        return Ok(true);
                    }
                }
                _ => {}
            }
        }

        Ok(false)
    }

    async fn try_notify(
        &self,
        store: &Store<Wry>,
        notification_ext: &Notification<Wry>,
        remote_manager: &RemoteManager,
    ) -> Result<(), String> {
        match self {
            WarpEvent::TransferAdded(remote_uuid, transfer_uuid) => {
                if !store
                    .get("notifications-transfers")
                    .map(|v| v.as_bool().unwrap_or(true))
                    .unwrap_or(true)
                {
                    return Ok(());
                }

                let transfer = remote_manager.transfer(remote_uuid, transfer_uuid).await;
                if let Some(transfer) = transfer {
                    let remote = remote_manager
                        .remote(remote_uuid)
                        .await
                        .expect("remote must exist if a transfer was found for its UUID");
                    let _ = notification_ext
                        .builder()
                        .title(format!("New transfer from {}", remote.display_name))
                        .body(format!("{} files", transfer.file_count))
                        .auto_cancel()
                        .show();
                }
            }
            WarpEvent::MessageAdded(remote_uuid, message_uuid) => {
                if !store
                    .get("notifications-messages")
                    .map(|v| v.as_bool().unwrap_or(true))
                    .unwrap_or(true)
                {
                    return Ok(());
                }

                let message = remote_manager.message(remote_uuid, message_uuid).await;
                if let Some(message) = message {
                    let remote = remote_manager
                        .remote(remote_uuid)
                        .await
                        .expect("remote must exist if a message was found for its UUID");
                    let _ = notification_ext
                        .builder()
                        .title(format!("New message from {}", remote.display_name))
                        .body(message.content)
                        .auto_cancel()
                        .show();
                }
            }
            _ => {}
        }
        Ok(())
    }
}
