use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use warpinator_lib::remote_manager::{RemoteManager, WarpEvent};
#[cfg(feature = "messaging")]
use warpinator_lib::types::message::Message;
use warpinator_lib::types::remote::{Remote, RemoteState};
use warpinator_lib::types::transfer::{Transfer, TransferState};

pub enum AppEvent {
    Terminal(ratatui::crossterm::event::Event),
    RemoteAdded(Remote),
    RemoteUpdated(Remote),
    TransferAdded(String, Transfer),   // remote_uuid, transfer
    TransferUpdated(String, Transfer), // remote_uuid, transfer
    TransferRemoved(String, String),   // remote_uuid, transfer_uuid
    Log(String),
    #[cfg(feature = "messaging")]
    MessageAdded(String, Message), // remote_uuid, message
    #[cfg(feature = "messaging")]
    MessageRemoved(String, String), // remote_uuid, message_uuid
}

impl AppEvent {
    pub async fn hydrate(ev: WarpEvent, rm: &RemoteManager) -> Option<Self> {
        match ev {
            WarpEvent::RemoteAdded(uuid) => Some(AppEvent::RemoteAdded(rm.remote(&uuid).await?)),
            WarpEvent::RemoteUpdated(uuid) => {
                Some(AppEvent::RemoteUpdated(rm.remote(&uuid).await?))
            }
            WarpEvent::TransferAdded(remote_uuid, transfer_uuid) => {
                let transfer = rm.transfer(&remote_uuid, &transfer_uuid).await?;
                Some(AppEvent::TransferAdded(remote_uuid, transfer))
            }
            WarpEvent::TransferUpdated(remote_uuid, transfer_uuid) => {
                let transfer = rm.transfer(&remote_uuid, &transfer_uuid).await?;
                Some(AppEvent::TransferUpdated(remote_uuid, transfer))
            }
            WarpEvent::TransferRemoved(remote_uuid, transfer_uuid) => {
                Some(AppEvent::TransferRemoved(remote_uuid, transfer_uuid))
            }
            #[cfg(feature = "messaging")]
            WarpEvent::MessageAdded(remote_uuid, message_uuid) => {
                let remote = rm.remote(&remote_uuid).await?;
                let message = remote.messages.iter().find(|m| m.uuid == message_uuid)?.clone();
                Some(AppEvent::MessageAdded(remote_uuid, message))
            }
            #[cfg(feature = "messaging")]
            WarpEvent::MessageRemoved(remote_uuid, message_uuid) => {
                Some(AppEvent::MessageRemoved(remote_uuid, message_uuid))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Focus {
    Remotes,
    Transfers,
    Messages,
}

#[derive(Debug, Clone)]
pub enum InputMode {
    FilePath,
    AcceptDestination,
    Message,
}

pub enum Action {
    None,
    Quit,
}

impl Action {
    pub fn is_quit(&self) -> bool {
        matches!(self, Action::Quit)
    }
}

pub struct App {
    pub remotes: Vec<Remote>,
    pub selected_remote: usize,
    pub transfers: HashMap<String, Vec<Transfer>>,
    pub selected_transfer: usize,
    pub selected_message: usize,
    pub focus: Focus,
    pub input_mode: Option<InputMode>,
    pub input_buf: String,
    pub log: VecDeque<String>,
    // Cache to throttle UI updates for transfer stats (to reduce flicker)
    pub transfer_display: HashMap<String, TransferDisplay>,
    // When the user starts an accept flow we store (remote_uuid, transfer_uuid)
    // here until they confirm or cancel.
    pub pending_accept: Option<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct TransferDisplay {
    pub last_update_ms: u128,
    pub displayed_bytes_transferred: u64,
    pub displayed_bytes_per_second: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            remotes: Vec::new(),
            selected_remote: 0,
            transfers: HashMap::new(),
            selected_transfer: 0,
            selected_message: 0,
            focus: Focus::Remotes,
            input_mode: None,
            input_buf: String::new(),
            log: VecDeque::with_capacity(200),
            transfer_display: HashMap::new(),
            pending_accept: None,
        }
    }

    pub fn log(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        if self.log.len() == 200 {
            self.log.pop_front();
        }
        self.log.push_back(msg);
    }

    pub fn current_remote(&self) -> Option<&Remote> {
        self.remotes.get(self.selected_remote)
    }

    pub fn current_transfers(&self) -> &[Transfer] {
        self.current_remote()
            .and_then(|r| self.transfers.get(&r.uuid))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn current_messages(&self) -> &[Message] {
        self.current_remote().map(|r| r.messages.as_slice()).unwrap_or(&[])
    }

    pub fn current_transfer(&self) -> Option<&Transfer> {
        self.current_transfers().get(self.selected_transfer)
    }

    pub fn current_message(&self) -> Option<&Message> {
        self.current_messages().get(self.selected_message)
    }

    pub fn handle_event(&mut self, ev: AppEvent) {
        match ev {
            AppEvent::RemoteAdded(remote) => {
                self.transfers.entry(remote.uuid.clone()).or_default();
                self.remotes.push(remote);
            }
            AppEvent::RemoteUpdated(remote) => {
                if let Some(r) = self.remotes.iter_mut().find(|r| r.uuid == remote.uuid) {
                    *r = remote;
                }
            }
            AppEvent::TransferAdded(remote_uuid, transfer) => {
                self.transfers.entry(remote_uuid).or_default().push(transfer.clone());
                // Initialize display cache for this transfer
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                self.transfer_display.insert(
                    transfer.uuid.clone(),
                    TransferDisplay {
                        last_update_ms: now,
                        displayed_bytes_transferred: transfer.bytes_transferred,
                        displayed_bytes_per_second: transfer.bytes_per_second,
                    },
                );
            }
            AppEvent::TransferUpdated(remote_uuid, transfer) => {
                if let Some(transfers) = self.transfers.get_mut(&remote_uuid) {
                    if let Some(t) = transfers.iter_mut().find(|t| t.uuid == transfer.uuid) {
                        *t = transfer.clone();
                        let throttle_ms: u128 = 750;
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|d| d.as_millis())
                            .unwrap_or(0);
                        let entry = self.transfer_display.entry(transfer.uuid.clone()).or_insert(
                            TransferDisplay {
                                last_update_ms: 0,
                                displayed_bytes_transferred: transfer.bytes_transferred,
                                displayed_bytes_per_second: transfer.bytes_per_second,
                            },
                        );
                        if now.saturating_sub(entry.last_update_ms) >= throttle_ms {
                            entry.last_update_ms = now;
                            entry.displayed_bytes_transferred = transfer.bytes_transferred;
                            entry.displayed_bytes_per_second = transfer.bytes_per_second;
                        }
                    }
                }
            }
            AppEvent::TransferRemoved(remote_uuid, transfer_uuid) => {
                if let Some(transfers) = self.transfers.get_mut(&remote_uuid) {
                    transfers.retain(|t| t.uuid != transfer_uuid);
                    if self.selected_transfer >= transfers.len() {
                        self.selected_transfer = transfers.len().saturating_sub(1);
                    }
                }
            }
            #[cfg(feature = "messaging")]
            AppEvent::MessageAdded(remote_uuid, message) => {
                if let Some(remote) = self.remotes.iter_mut().find(|r| r.uuid == remote_uuid) {
                    remote.messages.push(message);
                }
            }
            #[cfg(feature = "messaging")]
            AppEvent::MessageRemoved(remote_uuid, message_uuid) => {
                if let Some(remote) = self.remotes.iter_mut().find(|r| r.uuid == remote_uuid) {
                    remote.messages.retain(|m| m.uuid != message_uuid);
                    if self.selected_message >= remote.messages.len() {
                        self.selected_message = remote.messages.len().saturating_sub(1);
                    }
                }
            }
            AppEvent::Log(line) => {
                self.log(line);
            }
            AppEvent::Terminal(_) => {}
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        if self.input_mode.is_some() {
            return self.handle_input_key(key);
        }

        match key.code {
            KeyCode::Char('q') => return Action::Quit,
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Remotes => Focus::Transfers,
                    Focus::Transfers => Focus::Messages,
                    Focus::Messages => Focus::Remotes,
                };
            }
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('s') => {
                if self.current_remote().is_some() {
                    self.input_mode = Some(InputMode::FilePath);
                    self.input_buf.clear();
                }
            }
            KeyCode::Char('m') => {
                if self.current_remote().is_some() {
                    self.input_mode = Some(InputMode::Message);
                    self.input_buf.clear();
                }
            }
            KeyCode::Char('c') => {
                if self.focus == Focus::Remotes {
                    if let Some(remote) = self.current_remote() {
                        if !matches!(remote.state, RemoteState::Connected) {
                            self.log(format!("Connecting to {}...", remote.display_name)); // This is now handled in warpinator_tui.rs
                        }
                    }
                }
            }
            KeyCode::Char('a') | KeyCode::Char('r') | KeyCode::Char('x') | KeyCode::Delete => {
                self.handle_action_keys(key.code);
            }
            _ => {}
        }

        Action::None
    }

    fn handle_action_keys(&mut self, code: KeyCode) {
        match self.focus {
            Focus::Transfers => {
                if let Some(transfer) = self.current_transfer().cloned() {
                    match code {
                        KeyCode::Char('a') => {
                            if matches!(transfer.state, TransferState::WaitingPermission) {
                                self.log("Accepting transfer...".to_string()); // This is now handled in warpinator_tui.rs
                            }
                        }
                        KeyCode::Char('r') => {
                            if matches!(transfer.state, TransferState::WaitingPermission) {
                                self.log("Rejecting transfer...".to_string()); // This is now handled in warpinator_tui.rs
                            }
                        }
                        KeyCode::Char('x') => {
                            if matches!(transfer.state, TransferState::InProgress) {
                                self.log("Stopping transfer...".to_string()); // This is now handled in warpinator_tui.rs
                            }
                        }
                        KeyCode::Delete => {
                            if matches!(
                                transfer.state,
                                TransferState::Failed(_) | TransferState::Completed
                            ) {
                                self.log("Deleting transfer...".to_string()); // This is now handled in warpinator_tui.rs
                            }
                        }
                        _ => {}
                    }
                }
            }
            Focus::Messages => {
                if let Some(_message) = self.current_message().cloned() {
                    if code == KeyCode::Delete {
                        self.log("Deleting message...".to_string()); // This is now handled in warpinator_tui.rs
                    }
                }
            }
            _ => {}
        }
    }

    pub fn consume_input(&mut self) -> Option<(InputMode, String)> {
        if self.input_mode.is_some() {
            let mode = self.input_mode.take().unwrap();
            let buf = std::mem::take(&mut self.input_buf);
            return Some((mode, buf));
        }
        None
    }

    fn handle_input_key(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = None;
                self.input_buf.clear();
                // cancel any pending accept
                self.pending_accept = None;
            }
            KeyCode::Enter => {
                let value = std::mem::take(&mut self.input_buf);
                match self.input_mode.take() {
                    Some(InputMode::FilePath) => {
                        if let Some(remote) = self.current_remote() {
                            self.log(format!("send '{}' -> {}", value, remote.uuid)); // This is now handled in warpinator_tui.rs
                        }
                    }
                    Some(InputMode::AcceptDestination) => {
                        self.pending_accept = None;
                        self.log(format!("accept dest '{}'", value));
                    }
                    Some(InputMode::Message) => {
                        if let Some(remote) = self.current_remote() {
                            self.log(format!("msg '{}' -> {}", value, remote.uuid));
                        }
                    }
                    None => {}
                }
            }
            KeyCode::Backspace => {
                self.input_buf.pop();
            }
            KeyCode::Char(c) => {
                self.input_buf.push(c);
            }
            _ => {}
        }
        Action::None
    }

    fn move_down(&mut self) {
        match self.focus {
            Focus::Remotes => {
                if !self.remotes.is_empty() {
                    self.selected_remote = (self.selected_remote + 1) % self.remotes.len();
                    self.selected_transfer = 0;
                    self.selected_message = 0;
                }
            }
            Focus::Transfers => {
                let len = self.current_transfers().len();
                if len > 0 {
                    self.selected_transfer = (self.selected_transfer + 1) % len;
                }
            }
            Focus::Messages => {
                let len = self.current_messages().len();
                if len > 0 {
                    self.selected_message = (self.selected_message + 1) % len;
                }
            }
        }
    }

    fn move_up(&mut self) {
        match self.focus {
            Focus::Remotes => {
                if !self.remotes.is_empty() {
                    self.selected_remote =
                        self.selected_remote.checked_sub(1).unwrap_or(self.remotes.len() - 1);
                    self.selected_transfer = 0;
                    self.selected_message = 0;
                }
            }
            Focus::Transfers => {
                let len = self.current_transfers().len();
                if len > 0 {
                    self.selected_transfer =
                        self.selected_transfer.checked_sub(1).unwrap_or(len - 1);
                }
            }
            Focus::Messages => {
                let len = self.current_messages().len();
                if len > 0 {
                    self.selected_message = self.selected_message.checked_sub(1).unwrap_or(len - 1);
                }
            }
        }
    }
}
