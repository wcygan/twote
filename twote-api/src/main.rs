use crate::service::hello::HelloServiceImpl;
use crate::service::login::AccountServiceImpl;
use crate::service::profile::ProfileServiceImpl;
use common::middleware::authentication::AuthMiddleware;
use common::Service::TwoteApi;
use schemas::account::account_service_server::AccountServiceServer;
use schemas::hello::hello_service_server::HelloServiceServer;
use schemas::profile::profile_service_server::ProfileServiceServer;
use tonic::transport::Server;
use tracing::info;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the services
    let login_service = AccountServiceServer::new(AccountServiceImpl);
    let hello_service = HelloServiceServer::new(HelloServiceImpl);
    let profile_service = ProfileServiceServer::new(ProfileServiceImpl);
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    let addr = TwoteApi.socket_addr();
    info!("twote-api running on {}", addr);
    Server::builder()
        .layer(AuthMiddleware::default())
        .add_service(health_service)
        .add_service(login_service)
        .add_service(profile_service)
        .add_service(hello_service)
        .serve(addr)
        .await?;

    Ok(())
}
