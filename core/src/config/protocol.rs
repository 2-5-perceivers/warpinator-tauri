use std::time::Duration;

use crate::config::features::ProtocolFeatures;

const RECONNECT_INTERVAL_DEFAULT: Duration = Duration::from_secs(30);
const CONNECT_TIMEOUT_DEFAULT: Duration = Duration::from_secs(10);

const PING_INTERVAL_DEFAULT: Duration = Duration::from_secs(10);
const PING_TIMEOUT_DEFAULT: Duration = Duration::from_secs(5);

pub struct ProtocolConfigBuilder {
    features: Option<ProtocolFeatures>,
    reconnect_interval: Option<Duration>,
    connect_timeout: Option<Duration>,
    ping_interval: Option<Duration>,
    ping_timeout: Option<Duration>,
}

impl ProtocolConfigBuilder {
    fn new() -> Self {
        Self {
            features: None,
            reconnect_interval: None,
            connect_timeout: None,
            ping_interval: None,
            ping_timeout: None,
        }
    }

    pub fn features(mut self, features: ProtocolFeatures) -> Self {
        self.features = Some(features);
        self
    }

    pub fn reconnect_interval(mut self, interval: Duration) -> Self {
        self.reconnect_interval = Some(interval);
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    pub fn ping_interval(mut self, interval: Duration) -> Self {
        self.ping_interval = Some(interval);
        self
    }

    pub fn ping_timeout(mut self, timeout: Duration) -> Self {
        self.ping_timeout = Some(timeout);
        self
    }

    pub fn build(self) -> ProtocolConfig {
        ProtocolConfig {
            features: self.features.unwrap_or_default(),
            reconnect_interval: self.reconnect_interval.unwrap_or(RECONNECT_INTERVAL_DEFAULT),
            connect_timeout: self.connect_timeout.unwrap_or(CONNECT_TIMEOUT_DEFAULT),
            ping_interval: self.ping_interval.unwrap_or(PING_INTERVAL_DEFAULT),
            ping_timeout: self.ping_timeout.unwrap_or(PING_TIMEOUT_DEFAULT),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtocolConfig {
    pub features: ProtocolFeatures,
    pub reconnect_interval: Duration,
    pub connect_timeout: Duration,
    pub ping_interval: Duration,
    pub ping_timeout: Duration,
}

impl ProtocolConfig {
    pub fn builder() -> ProtocolConfigBuilder {
        ProtocolConfigBuilder::new()
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            features: ProtocolFeatures::default(),
            reconnect_interval: RECONNECT_INTERVAL_DEFAULT,
            connect_timeout: CONNECT_TIMEOUT_DEFAULT,
            ping_interval: PING_INTERVAL_DEFAULT,
            ping_timeout: PING_TIMEOUT_DEFAULT,
        }
    }
}
