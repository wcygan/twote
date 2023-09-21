use tonic::transport::Server;

use schemas::hello::hello_service_server::HelloServiceServer;
use schemas::login::login_service_server::LoginServiceServer;

use crate::service::hello::HelloServiceImpl;
use crate::service::login::LoginServiceImpl;

mod service;

const ADDR: &str = "0.0.0.0:8081";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let addr = ADDR.parse()?;

    // Create the services
    let login_service = LoginServiceServer::new(LoginServiceImpl::new().await?);
    let hello_service = HelloServiceServer::new(HelloServiceImpl);
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    println!("twote-api running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(hello_service)
        .add_service(login_service)
        .serve(addr)
        .await?;

    Ok(())
}
