#[macro_use]
extern crate log;

use tonic::transport::Server;

mod error;
mod proto;
mod service;

/// Start the RPC service.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("RPC service is starting.");

    Server::builder()
        .add_service(proto::accounts_server::AccountsServer::new(
            service::AccountsService::default(),
        ))
        .serve("0.0.0.0:50051".parse().unwrap())
        .await?;

    Ok(())
}
