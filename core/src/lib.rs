pub(crate) mod proto {
    tonic::include_proto!("warpinator");
}

pub mod config;
pub mod filesystem;
pub(crate) mod grpc;
pub(crate) mod server;
pub mod types;

#[cfg(feature = "power_manager")]
pub use server::power_manager;
pub use server::remote_worker::ConnectRemoteError;
pub use server::{WarpinatorBuildError, WarpinatorServeError, WarpinatorServer, remote_manager};
