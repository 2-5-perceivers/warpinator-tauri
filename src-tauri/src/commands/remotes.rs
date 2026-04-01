use std::net::IpAddr;

use serde::Serialize;
use tauri::State;
use warpinator_lib::remote_manager::RemoteManager;
use warpinator_lib::types::remote::{Remote, RemoteState};

#[derive(Serialize, Clone, Debug)]
pub struct RemoteUi {
    pub uuid: String,
    pub ip: IpAddr,
    pub port: u16,
    pub auth_port: u16,
    pub service_name: String,
    pub display_name: String,
    pub username: String,
    pub hostname: String,
    pub picture: Option<String>,
    pub state: RemoteState,
    pub service_static: bool,
    pub service_available: bool,
}

impl From<Remote> for RemoteUi {
    fn from(remote: Remote) -> Self {
        Self {
            uuid: remote.uuid.clone(),
            ip: remote.ip,
            port: remote.port,
            auth_port: remote.auth_port,
            service_name: remote.service_name,
            display_name: remote.display_name,
            username: remote.username,
            hostname: remote.hostname,
            picture: remote.picture.map(|_| format!("avatars://{}", remote.uuid)),
            state: remote.state,
            service_static: remote.service_static,
            service_available: remote.service_available,
        }
    }
}

#[tauri::command]
pub async fn get_remote(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
) -> Result<Option<RemoteUi>, String> {
    Ok(remotes.remote(remote_uuid.as_str()).await.map(|r| r.into()))
}

#[tauri::command]
pub async fn get_remotes(remotes: State<'_, RemoteManager>) -> Result<Vec<RemoteUi>, String> {
    Ok(remotes.remotes().await.into_iter().map(|r| r.into()).collect())
}

#[tauri::command]
pub async fn connect_remote(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
) -> Result<(), String> {
    let remote = remotes.get_worker(remote_uuid.as_str()).await.ok_or("No remote found")?;
    remote.connect().await.map_err(|e| e.to_string())
}
