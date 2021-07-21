use tonic::transport::Server;

mod error;
mod proto;
mod service;

/// Start the RPC service.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(proto::accounts_server::AccountsServer::new(
            service::AccountsService::default(),
        ))
        .serve("[::1]:50051".parse().unwrap())
        .await?;

    println!("RPC service has been started.");

    Ok(())
}
