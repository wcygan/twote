use common::Service::{TweetsBackend};
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the services
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    let addr = TweetsBackend.socket_addr();
    info!("tweets-backend is running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .serve(addr)
        .await?;

    Ok(())
}