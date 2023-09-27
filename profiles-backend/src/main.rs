use tonic::transport::Server;

use common::Service::ProfilesBackend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the services
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    let addr = ProfilesBackend.socket_addr();
    println!("profiles-backend is running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .serve(addr)
        .await?;

    Ok(())
}
