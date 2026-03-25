use std::sync::Arc;

use tauri::{Emitter, Manager, State};
use tokio::sync::oneshot;
use warpinator_lib::config::user::UserConfig;
use warpinator_lib::remote_manager::{RemoteManager, WarpEvent};
use warpinator_lib::types::remote::Remote;
use warpinator_lib::WarpinatorServer;

#[tauri::command]
async fn get_remotes(remotes: State<'_, RemoteManager>) -> Result<Vec<Remote>, String> {
    Ok(remotes.remotes().await)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![get_remotes])
        .setup(|app| {
            let handle = app.handle().clone();
            let user_config = UserConfig::builder()
                .default_bind_addr_v4()
                .default_bind_addr_v6()
                .default_hostname()
                .build();

            let service_id = "warpinator-tauri";

            let server = WarpinatorServer::builder()
                .user_config(user_config)
                .service_name(&service_id)
                .build()
                .expect("failed to build server");

            let remote_manager = server.remotes.clone();
            let mut warp_events = server.remotes.subscribe();
            let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

            // store what you need to reach from commands
            handle.manage(remote_manager);
            handle.manage(Arc::new(shutdown_tx));

            tauri::async_runtime::spawn(async move {
                // forward lib events to frontend
                let event_handle = handle.clone();
                tokio::spawn(async move {
                    while let Ok(ev) = warp_events.recv().await {
                        let _ = event_handle.emit("warp-event", &ev);
                    }
                });

                server
                    .serve_with_shutdown(async move {
                        let _ = shutdown_rx.await;
                    })
                    .await
                    .expect("server error");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
