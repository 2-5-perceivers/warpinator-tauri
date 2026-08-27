use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tonic::{Request, Response, Status};
use tracing::{field, instrument};

use crate::proto::warp_registration_server::WarpRegistration;
use crate::proto::{RegRequest, RegResponse, ServiceRegistration};
use crate::server::authenticator::Authenticator;
use crate::server::remote_manager::RemoteManager;
use crate::types::remote::{Remote, RemoteState};

#[derive(Debug)]
pub struct RegistrationServer {
    authenticator: Arc<Authenticator>,
    remote_manager: RemoteManager,
    reg_service_message: ServiceRegistration,
}

impl RegistrationServer {
    pub fn new(
        authenticator: Arc<Authenticator>,
        remote_manager: RemoteManager,
        reg_service_message: ServiceRegistration,
    ) -> Self {
        Self { authenticator, remote_manager, reg_service_message }
    }
}

#[tonic::async_trait]
impl WarpRegistration for RegistrationServer {
    #[instrument(
        skip(self, request),
        level = "debug",
        fields(
            ip = field::Empty,
            hostname = field::Empty
        ),
        err
    )]
    async fn request_certificate(
        &self,
        request: Request<RegRequest>,
    ) -> Result<Response<RegResponse>, Status> {
        let req = request.into_inner();

        let span = tracing::Span::current();
        span.record("ip", &req.ip);
        span.record("hostname", &req.hostname);

        let boxed = self
            .authenticator
            .box_cert()
            .map_err(|_| Status::internal("Failed to box certificate"))?;

        let locked_cert = STANDARD.encode(&boxed);

        Ok(Response::new(RegResponse { locked_cert }))
    }

    #[instrument(
        skip(self, request),
        level = "debug",
        fields(
            service_id = field::Empty,
            ip = field::Empty,
            port = field::Empty,
            auth_port = field::Empty,
            hostname = field::Empty
        ),
        err
    )]
    async fn register_service(
        &self,
        request: Request<ServiceRegistration>,
    ) -> Result<Response<ServiceRegistration>, Status> {
        let req = request.into_inner();

        let span = tracing::Span::current();
        span.record("ip", &req.ip);
        span.record("hostname", &req.hostname);
        span.record("service_id", &req.service_id);
        span.record("port", req.port);
        span.record("auth_port", req.auth_port);

        let remote = self.remote_manager.remote(req.service_id.as_ref()).await;

        match remote {
            None => {
                tracing::info!("Registered new remote service");
                let new_remote = Remote::new(
                    req.service_id.clone(),
                    IpAddr::from_str(&req.ip)
                        .map_err(|e| Status::invalid_argument(e.to_string()))?,
                    req.port.try_into().unwrap(),
                    req.auth_port.try_into().unwrap(),
                    req.service_id.clone(),
                    req.hostname.clone(),
                );
                self.remote_manager.add_remote(new_remote).await;
            }
            Some(remote) if remote.state == RemoteState::Connected => {
                tracing::warn!("Attempted registration from already connected remote",);
            }
            Some(remote) => {
                let needs_reconnect =
                    matches!(remote.state, RemoteState::Disconnected | RemoteState::Error(_));

                let ip = IpAddr::from_str(&req.ip)
                    .map_err(|e| Status::invalid_argument(e.to_string()))?;

                self.remote_manager
                    .update_remote(&req.service_id, |remote| {
                        remote.ip = ip;
                        remote.port = req.port.try_into().unwrap();
                        remote.auth_port = req.auth_port.try_into().unwrap();
                        remote.hostname = req.hostname.clone();
                        remote.service_name = req.service_id.clone();
                    })
                    .await
                    .unwrap();

                if needs_reconnect {
                    tracing::info!(
                        "Disconnected or errored remote re-registered, triggering reconnect"
                    );
                    self.remote_manager
                        .get_worker(&req.service_id)
                        .await
                        .ok_or(Status::internal(
                            "Failed to connect to remote after registration update",
                        ))?
                        .connect()
                        .await
                        .map_err(|e| {
                            Status::internal(format!(
                                "Failed to connect to remote after registration update: {e}"
                            ))
                        })?;
                }
            }
        }

        Ok(Response::new(self.reg_service_message.clone()))
    }
}
