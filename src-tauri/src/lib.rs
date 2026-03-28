use std::sync::Arc;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri::image::Image;
use tokio::sync::oneshot;
use warpinator_lib::WarpinatorServer;
use warpinator_lib::config::user::UserConfig;
use warpinator_lib::remote_manager::RemoteManager;
use warpinator_lib::types::remote::Remote;

#[tauri::command]
async fn get_remotes(remotes: State<'_, RemoteManager>) -> Result<Vec<Remote>, String> {
    Ok(remotes.remotes().await)
}

fn spawn_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let icon = include_bytes!("../icons/symbolic.png");

    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    let _ = TrayIconBuilder::new()
        .icon(Image::from_bytes(icon)?)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                let window = app.get_webview_window("main").expect("no main window");
                let _ = window.show();
                let _ = window.set_focus();
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let window = app.get_webview_window("main").expect("no main window");
            let _ = window.show();
            let _ = window.set_focus();
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
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
            let event_handle = handle.clone();

            tauri::async_runtime::spawn(async move {
                // forward lib events to frontend
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

            tauri::async_runtime::spawn_blocking(move || {
                let _ = spawn_tray(&handle);
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window.hide().unwrap();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
