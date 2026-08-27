mod tui;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc as std_mpsc};
use std::time::Duration;
use std::{env, fs, io};

use anyhow::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use sha2::{Digest, Sha256};
use tokio::sync::{mpsc, oneshot};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tui::app::{App, AppEvent, Focus, InputMode};
use warpinator_lib::WarpinatorServer;
use warpinator_lib::config::user::UserConfig;
use warpinator_lib::remote_manager::RemoteManager;
use warpinator_lib::types::remote::RemoteState;
use warpinator_lib::types::transfer::TransferState;

#[derive(Clone)]
struct TuiLogWriter {
    tx: mpsc::Sender<AppEvent>,
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for TuiLogWriter {
    type Writer = TuiLogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

impl io::Write for TuiLogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(s) = std::str::from_utf8(buf) {
            let line = s.trim_end_matches('\n').to_string();
            if !line.is_empty() {
                let _ = self.tx.try_send(AppEvent::Log(line));
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<AppEvent>(512);

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

    let server = WarpinatorServer::builder()
        .user_config(user_config)
        .service_name(&service_id)
        .build()
        .expect("failed to build server");

    let remote_manager = server.remotes.clone();
    let mut warp_events = server.remotes.subscribe();

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        server
            .serve_with_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("server error");
    });

    let tx_term = tx.clone();
    let (term_shutdown_tx, term_shutdown_rx) = std_mpsc::channel::<()>();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let term_handle = tokio::task::spawn_blocking(move || {
        while running_clone.load(Ordering::Relaxed) {
            if let Ok(_) = term_shutdown_rx.try_recv() {
                break;
            }
            if let Ok(ev) = ratatui::crossterm::event::read() {
                if tx_term.blocking_send(AppEvent::Terminal(ev)).is_err() {
                    break;
                }
            }
        }
    });

    let tx_lib = tx.clone();
    let rm_for_events = remote_manager.clone();
    tokio::spawn(async move {
        while let Ok(ev) = warp_events.recv().await {
            if let Some(app_ev) = AppEvent::hydrate(ev, &rm_for_events).await {
                if tx_lib.send(app_ev).await.is_err() {
                    break;
                }
            }
        }
    });

    let tui_writer = TuiLogWriter { tx: tx.clone() };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(tui_writer).with_ansi(false))
        .with(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.log("warpinator-tui started");

    let result = run(&mut terminal, &mut app, &mut rx, remote_manager).await;

    running.store(false, Ordering::Relaxed);
    let _ = term_shutdown_tx.send(());
    let _ = term_handle.await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    let _ = shutdown_tx.send(());

    result
}

async fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    rx: &mut mpsc::Receiver<AppEvent>,
    remote_manager: RemoteManager,
) -> Result<()> {
    let mut draw_tick = tokio::time::interval(Duration::from_millis(33)); // ~30fps

    loop {
        tokio::select! {
            _ = draw_tick.tick() => {
                terminal.draw(|f| tui::ui::draw(f, app))?;
            }
            ev = rx.recv() => {
                match ev {
                    Some(AppEvent::Terminal(CEvent::Key(key))) => {
                        let mut handled = false;

                        if key.code == KeyCode::Enter {
                            if let Some((captured_mode, captured_buf)) = app.consume_input() {
                                if app.handle_key(key).is_quit() {
                                    break;
                                }

                                match captured_mode {
                                    InputMode::Message => {
                                        if let Some(remote) = app.current_remote() {
                                            let remote_uuid = remote.uuid.clone();
                                            let msg = captured_buf;
                                            let rm = remote_manager.clone();
                                            tokio::spawn(async move {
                                                if let Some(worker) = rm.get_worker(&remote_uuid).await {
                                                    let _ = worker.send_message(&msg).await;
                                                }
                                            });
                                        }
                                        handled = true;
                                    }
                                    InputMode::FilePath => {
                                        if let Some(remote) = app.current_remote() {
                                            let value = captured_buf;
                                            let remote_uuid = remote.uuid.clone();
                                            let rm = remote_manager.clone();
                                            tokio::spawn(async move {
                                                if let Some(worker) = rm.get_worker(&remote_uuid).await {
                                                    let paths: Vec<std::path::PathBuf> = value
                                                        .split(';')
                                                        .map(|s| s.trim())
                                                        .filter(|s| !s.is_empty())
                                                        .map(|s| std::path::PathBuf::from(s))
                                                        .collect();
                                                    if !paths.is_empty() {
                                                        let _ = worker.send_transfer_request(paths).await;
                                                    }
                                                }
                                            });
                                        }
                                        handled = true;
                                    }
                                    InputMode::AcceptDestination => {
                                        if let Some((remote_uuid, transfer_uuid)) = app.pending_accept.take() {
                                            let value = captured_buf;
                                            let rm = remote_manager.clone();
                                            tokio::spawn(async move {
                                                if let Some(worker) = rm.get_worker(&remote_uuid).await {
                                                    let dest = std::path::PathBuf::from(value);
                                                    let _ = worker.accept_transfer(&transfer_uuid, dest).await;
                                                }
                                            });
                                        }
                                        handled = true;
                                    }
                                }
                            }
                        }

                        if key.code == KeyCode::Char('a') {
                            if app.input_mode.is_none() && app.focus == Focus::Transfers {
                                if let Some((remote_uuid, transfer_uuid)) = (|| {
                                    if let Some(remote) = app.current_remote() {
                                        if let Some(t) = app.current_transfers().get(app.selected_transfer) && matches!(t.state, TransferState::WaitingPermission) && matches!(t.kind, warpinator_lib::types::transfer::TransferKind::Incoming { .. }) {
                                            return Some((remote.uuid.clone(), t.uuid.clone()));
                                        }
                                    }
                                    None
                                })() {
                                    let dest = std::env::var("WARPINATOR_DIR").ok().unwrap_or_else(|| {
                                        env::current_dir()
                                            .map(|p| p.to_string_lossy().to_string())
                                            .unwrap_or_else(|_| ".".to_string())
                                    });
                                    app.input_mode = Some(InputMode::AcceptDestination);
                                    app.input_buf = dest;
                                    app.pending_accept = Some((remote_uuid, transfer_uuid));
                                    handled = true;
                                }
                            }
                        }

                        if key.code == KeyCode::Char('r') {
                            if app.input_mode.is_none() && app.focus == Focus::Transfers {
                                if let Some(transfer) = app.current_transfer() {
                                    if matches!(transfer.state, TransferState::WaitingPermission) {
                                        let remote_uuid = app.current_remote().unwrap().uuid.clone();
                                        let transfer_uuid = transfer.uuid.clone();
                                        let rm = remote_manager.clone();
                                        tokio::spawn(async move {
                                            if let Some(worker) = rm.get_worker(&remote_uuid).await {
                                                let _ = worker.cancel_transfer(&transfer_uuid).await;
                                            }
                                        });
                                        handled = true;
                                    }
                                }
                            }
                        }

                        if key.code == KeyCode::Char('c') {
                            if app.input_mode.is_none() && app.focus == Focus::Remotes {
                                if let Some(remote) = app.current_remote() {
                                    if matches!(remote.state, RemoteState::Disconnected | RemoteState::Error(_)) {
                                        let remote_uuid = remote.uuid.clone();
                                        let rm = remote_manager.clone();
                                        tokio::spawn(async move {
                                            if let Some(worker) = rm.get_worker(&remote_uuid).await {
                                                let _ = worker.connect().await;
                                            }
                                        });
                                        handled = true;
                                    }
                                }
                            }
                        }

                        if key.code == KeyCode::Char('x') {
                            if app.input_mode.is_none() && app.focus == Focus::Transfers {
                                if let Some(transfer) = app.current_transfer() {
                                    if matches!(transfer.state, TransferState::InProgress) {
                                        let remote_uuid = app.current_remote().unwrap().uuid.clone();
                                        let transfer_uuid = transfer.uuid.clone();
                                        let rm = remote_manager.clone();
                                        tokio::spawn(async move {
                                            if let Some(worker) = rm.get_worker(&remote_uuid).await {
                                                let _ = worker.stop_transfer(&transfer_uuid, false).await;
                                            }
                                        });
                                        handled = true;
                                    }
                                }
                            }
                        }

                        if key.code == KeyCode::Delete {
                            if app.input_mode.is_none() {
                                match app.focus {
                                    Focus::Transfers => {
                                        if let Some(transfer) = app.current_transfer() {
                                            if matches!(transfer.state, TransferState::Failed(_)
                            | TransferState::Completed
                            | TransferState::Stopped
                            | TransferState::Denied
                            | TransferState::Canceled) {
                                                let remote_uuid = app.current_remote().unwrap().uuid.clone();
                                                let transfer_uuid = transfer.uuid.clone();
                                                let rm = remote_manager.clone();
                                                tokio::spawn(async move {
                                                    let _ = rm.remove_transfer(&remote_uuid, &transfer_uuid).await;
                                                });
                                                handled = true;
                                            }
                                        }
                                    }
                                    Focus::Messages => {
                                        if let Some(message) = app.current_message() {
                                            let remote_uuid = app.current_remote().unwrap().uuid.clone();
                                            let message_uuid = message.uuid.clone();
                                            let rm = remote_manager.clone();
                                            tokio::spawn(async move {
                                                let _ = rm.remove_message(&remote_uuid, &message_uuid).await;
                                            });
                                            handled = true;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        if !handled {
                            if app.handle_key(key).is_quit() {
                                break;
                            }
                        }
                     }
                    Some(ev) => app.handle_event(ev),
                    None => break,
                }
            }
        }
    }

    Ok(())
}
