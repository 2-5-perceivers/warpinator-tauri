use mdns_sd::{ResolvedService, ServiceDaemon, ServiceEvent};
use thiserror::Error;
use tracing::{field, instrument};

use crate::server::remote_manager;
use crate::server::remote_worker::ConnectRemoteError;
use crate::types::remote::{Remote, RemoteState};

#[derive(Error, Debug)]
enum DiscoveryNewServiceError {
    #[error("No valid IP address found for service")]
    NoValidIpAddress,
    #[error("API version {0} is not supported")]
    UnsupportedApiVersion(u8), // (remote api version)
    #[error("Flush registration")]
    FlushRegistration,
    #[error("Service {0} property missing")]
    MissingTxtProperty(String), // (property name)
    #[error("Service {0} property is not valid: {1}")]
    InvalidTxtProperty(String, String), // (property name, property value)
    #[error("Failed to connect to remote after registration update: {0}")]
    FailedToConnectToRemote(ConnectRemoteError), // (error message)
}

pub struct DiscoveryService {
    remote_manager: remote_manager::RemoteManager,
    mdns: ServiceDaemon,
    service_domain: String,
    #[cfg(feature = "power_manager")]
    power_manager: std::sync::Arc<dyn crate::server::power_manager::PowerManager>,
}

impl DiscoveryService {
    pub fn new(
        remote_manager: remote_manager::RemoteManager,
        mdns: ServiceDaemon,
        service_domain: String,
        #[cfg(feature = "power_manager")] power_manager: std::sync::Arc<
            dyn crate::server::power_manager::PowerManager,
        >,
    ) -> Self {
        Self {
            remote_manager,
            mdns,
            service_domain,
            #[cfg(feature = "power_manager")]
            power_manager,
        }
    }

    #[instrument(skip(self), err)]
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.mdns.set_multicast_loop_v4(false)?;
        self.mdns.set_multicast_loop_v6(false)?;

        let browse_rx = self.mdns.browse(&self.service_domain)?;

        while let Ok(event) = browse_rx.recv_async().await {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    let _ = self.handle_new_service(*info).await;
                }
                ServiceEvent::ServiceRemoved(_, fullname) => {
                    tracing::info!("Service removed: {}", fullname);
                    let _ = self
                        .remote_manager
                        .update_remote(fullname.as_str(), |remote| {
                            remote.service_available = false;
                            remote.state = RemoteState::Disconnected;
                        })
                        .await;
                }
                _ => {}
            }
        }

        Ok(())
    }

    #[instrument(
        skip(self, resolved_service),
        fields(
            service_name = field::Empty,
            address = field::Empty,
        ),
        err(level = "warn")
    )]
    async fn handle_new_service(
        &self,
        resolved_service: ResolvedService,
    ) -> Result<(), DiscoveryNewServiceError> {
        // Lock the cpu so the connection is handled right
        #[cfg(feature = "power_manager")]
        let _wake_lock =
            crate::server::power_manager::WakeLockGuard::new(self.power_manager.clone());

        let fullname = resolved_service.get_fullname();
        let name = fullname
            .trim_end_matches(&format!(".{}", resolved_service.ty_domain))
            .trim_end_matches('.');

        let span = tracing::Span::current();
        span.record("service_name", name);

        let addresses = resolved_service.get_addresses();
        let address_option = addresses.iter().find(|addr| addr.is_ipv4());
        // .find(|addr| addr.is_ipv6())
        // .or_else(|| addresses.iter().find(|addr| addr.is_ipv4()));
        let address =
            address_option.ok_or(DiscoveryNewServiceError::NoValidIpAddress)?.to_ip_addr();

        span.record("address", address.to_string());

        let port = resolved_service.port;
        let txt_properties = resolved_service.get_properties();

        let auth_port_str = txt_properties
            .get("auth-port")
            .ok_or(DiscoveryNewServiceError::MissingTxtProperty("auth-port".into()))?
            .val_str();
        let auth_port = auth_port_str.parse::<u16>().map_err(|_| {
            DiscoveryNewServiceError::InvalidTxtProperty("auth-port".into(), auth_port_str.into())
        })?;

        let hostname = txt_properties
            .get("hostname")
            .ok_or(DiscoveryNewServiceError::MissingTxtProperty("hostname".into()))?
            .val_str();

        // Assume remote is using API v1 if "api-version" property is missing
        let fallback_api_version =
            mdns_sd::TxtProperty::from(("api-version".to_string(), "1".to_string()));
        let api_version_str =
            txt_properties.get("api-version").unwrap_or(&fallback_api_version).val_str();
        let api_version = api_version_str.parse::<u8>().map_err(|_| {
            DiscoveryNewServiceError::InvalidTxtProperty(
                "api-version".into(),
                api_version_str.into(),
            )
        })?;

        if api_version != 2 {
            return Err(DiscoveryNewServiceError::UnsupportedApiVersion(api_version));
        }

        let service_type_option = txt_properties.get("type");

        if let Some(service_type) = service_type_option {
            let service_type_str = service_type.val_str();
            if service_type_str == "flush" {
                return Err(DiscoveryNewServiceError::FlushRegistration);
            }
        }

        tracing::info!("Resolved mDNS service");

        let remote = self.remote_manager.remote(name).await;

        match remote {
            None => {
                let mut remote = Remote::new(
                    name.into(),
                    address,
                    port,
                    auth_port,
                    name.into(),
                    hostname.into(),
                );

                remote.service_available = true;

                self.remote_manager
                    .add_remote(remote)
                    .await
                    .connect()
                    .await
                    .map_err(DiscoveryNewServiceError::FailedToConnectToRemote)?;
            }
            Some(remote) => {
                tracing::debug!("MDNS service corresponds to existing remote, updating info");
                let _ = self
                    .remote_manager
                    .update_remote(name, |remote| {
                        remote.hostname = hostname.into();
                        remote.auth_port = auth_port;
                        remote.ip = address;
                        remote.port = port;
                        remote.service_available = true;
                    })
                    .await;

                if matches!(remote.state, RemoteState::Disconnected | RemoteState::Error(_)) {
                    tracing::info!(
                        "Previously disconnected remote is now available, attempting to connect"
                    );
                    self.remote_manager
                        .get_worker(&remote.uuid)
                        .await
                        .ok_or(DiscoveryNewServiceError::FailedToConnectToRemote(
                            ConnectRemoteError::RemoteWorkerNotFound,
                        ))?
                        .connect()
                        .await
                        .map_err(DiscoveryNewServiceError::FailedToConnectToRemote)?;
                }
            }
        }

        Ok(())
    }
}
