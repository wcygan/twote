use tonic::transport::Server;
use tracing::info;

use crate::service::home_page::HomePageServiceImpl;
use crate::service::login::AccountServiceImpl;
use crate::service::profile_page::ProfilePageServiceImpl;
use common::authentication::AuthMiddleware;
use common::Service::TwoteApi;
use schemas::account::account_service_server::AccountServiceServer;
use schemas::frontend::home_page_service_server::HomePageServiceServer;
use schemas::frontend::profile_page_service_server::ProfilePageServiceServer;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the services
    let home_page_service = HomePageServiceServer::new(HomePageServiceImpl);
    let profile_page_service = ProfilePageServiceServer::new(ProfilePageServiceImpl);
    let account_service = AccountServiceServer::new(AccountServiceImpl);
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    let addr = TwoteApi.socket_addr();
    info!("twote-api running on {}", addr);
    Server::builder()
        .layer(AuthMiddleware::default())
        .add_service(home_page_service)
        .add_service(profile_page_service)
        .add_service(account_service)
        .add_service(health_service)
        .serve(addr)
        .await?;

    Ok(())
}
