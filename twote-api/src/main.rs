use crate::service::hello::HelloServiceImpl;
use crate::service::login::AccountServiceImpl;
use common::middleware::authentication::AuthMiddleware;
use common::Service::TwoteApi;
use schemas::account::account_service_server::AccountServiceServer;
use schemas::hello::hello_service_server::HelloServiceServer;
use tonic::transport::Server;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let addr = TwoteApi.socket_addr();

    // Create the services
    let login_service = AccountServiceServer::new(AccountServiceImpl);
    let hello_service = HelloServiceServer::new(HelloServiceImpl);
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    println!("twote-api running on {}", addr);
    Server::builder()
        .layer(AuthMiddleware::default())
        .add_service(health_service)
        .add_service(login_service)
        .add_service(hello_service)
        .serve(addr)
        .await?;

    Ok(())
}
