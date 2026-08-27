use std::net::IpAddr;
use std::sync::Arc;

use thiserror::Error;
use tokio::sync::RwLock;

use crate::config::features::ProtocolFeatures;
#[cfg(feature = "messaging")]
use crate::types::message::Message;
use crate::types::transfer::Transfer;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum RemoteConnectionError {
    #[error("SSL connection failed")]
    SslError,
    #[error("Group code mismatch")]
    GroupCodeMismatch,
    #[error("No certificate")]
    NoCertificate,
    #[error("Duplex connection failed")]
    DuplexError,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "snake_case"),
    serde(tag = "type", content = "content")
)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RemoteState {
    Error(RemoteConnectionError),
    Disconnected,
    Connecting,
    AwaitingDuplex,
    Connected,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug)]
pub struct Remote {
    pub uuid: String,
    pub ip: IpAddr,
    pub port: u16,
    pub auth_port: u16,
    pub service_name: String,
    pub features: ProtocolFeatures,

    pub display_name: String,
    pub username: String,
    pub hostname: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub picture: Option<Arc<RwLock<Vec<u8>>>>,
    /// A small number that changes everytime the data does. Do not use it to
    /// cache old versions.
    pub picture_version: u8,

    pub state: RemoteState,

    #[cfg_attr(feature = "serde", serde(skip))]
    #[cfg(feature = "messaging")]
    pub messages: Vec<Message>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub transfers: Vec<Transfer>,

    /// Whether the remote's service is static (i.e. registered) or dynamic
    /// (i.e. discovered on the network)
    pub service_static: bool,
    /// Whether the remote's mdns service is currently available
    pub service_available: bool,
    /// Unboxed PEM certificate for the remote
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) cert_pem: Option<Vec<u8>>,
}

impl Remote {
    pub fn new(
        uuid: String,
        ip: IpAddr,
        port: u16,
        auth_port: u16,
        service_name: String,
        hostname: String,
    ) -> Self {
        Self {
            uuid,
            ip,
            port,
            auth_port,
            service_name,
            features: ProtocolFeatures::empty(),
            display_name: "".to_string(),
            username: "".to_string(),
            hostname,
            picture: None,
            picture_version: 0,
            state: RemoteState::Disconnected,
            #[cfg(feature = "messaging")]
            messages: vec![],
            transfers: vec![],
            service_static: false,
            service_available: false,
            cert_pem: None,
        }
    }
}
