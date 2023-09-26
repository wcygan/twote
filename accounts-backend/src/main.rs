use crate::service::login::AccountServiceImpl;
use common::Service::AccountsBackend;
use schemas::account::account_service_server::AccountServiceServer;
use tonic::transport::Server;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let addr = AccountsBackend.socket_addr();

    // Create the services
    let login_service = AccountServiceServer::new(AccountServiceImpl);
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    println!("accounts-backend is running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(login_service)
        .serve(addr)
        .await?;

    Ok(())
}
