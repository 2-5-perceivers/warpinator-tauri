use std::{env, fs};

use sha2::{Digest, Sha256};
use tracing_subscriber::fmt::format::FmtSpan;
use warpinator_lib::config::user::UserConfig;
use warpinator_lib::remote_manager::WarpEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Warpinator starting...");

    let group_code = env::var("WARPINATOR_GROUP_CODE").unwrap_or_else(|_| "Warpinator".to_string());
    let display_name =
        env::var("WARPINATOR_DISPLAY_NAME").unwrap_or_else(|_| "Warpinator RS".to_string());
    let picture = env::var("WARPINATOR_PICTURE").ok().and_then(|path| fs::read(path).ok());
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .or_else(|_| env::var("WARPINATOR_USERNAME"))
        .unwrap_or_else(|_| "warpinator-rs".to_string());
    let hostname = hostname::get()
        .ok()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|| "warpinator".to_string());
    let mut hasher = Sha256::new();
    hasher.update(hostname.as_bytes());
    hasher.update(username.as_bytes());
    hasher.update(group_code.as_bytes());
    hasher.update(display_name.as_bytes());
    let hash = hasher.finalize();
    let service_id = format!(
        "WARPINATOR-{:X}",
        u64::from_be_bytes([
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]
        ])
    );

    let mut user_config_builder = UserConfig::builder()
        .default_bind_addr_v4()
        .default_bind_addr_v6()
        .hostname(&hostname)
        .username(&username)
        .display_name(&display_name)
        .group_code(&group_code);
    if let Some(pic) = picture {
        user_config_builder = user_config_builder.picture(&pic);
    }
    let user_config = user_config_builder.build();

    let server = warpinator_lib::WarpinatorServer::builder()
        .user_config(user_config)
        .service_name(service_id.as_str())
        .build()?;

    // grab subscriber before serve consumes the server
    let remote_manger = server.remotes.clone();
    let mut events = server.remotes.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            match event {
                WarpEvent::RemoteAdded(uuid) => {
                    let remote = remote_manger
                        .clone()
                        .remote(uuid.as_str())
                        .await
                        .expect("Event for non-existent remote");
                    tracing::info!("Remote added: {:?}", remote)
                }
                WarpEvent::RemoteUpdated(uuid) => {
                    let remote = remote_manger
                        .clone()
                        .remote(uuid.as_str())
                        .await
                        .expect("Event for non-existent remote");
                    println!("Remote updated: {} with {:?}", uuid, remote.state)
                }
                WarpEvent::TransferAdded(remote_uuid, transfer_uuid) => {
                    let transfer = remote_manger
                        .clone()
                        .transfer(&remote_uuid, &transfer_uuid)
                        .await
                        .expect("Event for non-existent transfer");
                    println!(
                        "Transfer added: {} on {}\n{:?}",
                        transfer_uuid, remote_uuid, transfer
                    );
                }
                WarpEvent::TransferUpdated(remote_uuid, transfer_uuid) => {
                    println!("Transfer updated: {} on {}", transfer_uuid, remote_uuid)
                }
                WarpEvent::TransferRemoved(remote_uuid, transfer_uuid) => {
                    println!("Transfer removed: {} on {}", transfer_uuid, remote_uuid)
                }
                WarpEvent::MessageAdded(_, _) => {}
                WarpEvent::MessageRemoved(_, _) => {}
                _ => {}
            }
        }
    });

    server
        .serve_with_shutdown(async {
            tokio::signal::ctrl_c().await.expect("failed to listen for ctrl-c");
        })
        .await?;

    println!("Shutting down...");
    Ok(())
}
