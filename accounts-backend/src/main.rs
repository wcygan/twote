use crate::service::login::LoginServiceImpl;
use schemas::login::login_service_server::LoginServiceServer;
use tonic::transport::Server;

mod service;

const ADDR: &str = "127.0.0.1:8082";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ADDR.parse()?;

    // Create the services
    let login_service = LoginServiceServer::new(LoginServiceImpl);
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
