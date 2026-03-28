use tauri::State;
use warpinator_lib::remote_manager::RemoteManager;
use warpinator_lib::types::remote::Remote;

#[tauri::command]
pub async fn get_remote(
    remotes: State<'_, RemoteManager>,
    remote_uuid: String,
) -> Result<Option<Remote>, String> {
    Ok(remotes.remote(remote_uuid.as_str()).await)
}

#[tauri::command]
pub async fn get_remotes(remotes: State<'_, RemoteManager>) -> Result<Vec<Remote>, String> {
    Ok(remotes.remotes().await)
}
