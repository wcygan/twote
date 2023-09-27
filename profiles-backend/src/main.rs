use tonic::transport::Server;

use crate::service::profile::ProfileServiceImpl;
use common::Service::ProfilesBackend;
use schemas::profile::profile_service_server::ProfileServiceServer;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the services
    let (_, health_service) = tonic_health::server::health_reporter();
    let profile_service = ProfileServiceServer::new(ProfileServiceImpl);

    // Start the server
    let addr = ProfilesBackend.socket_addr();
    println!("profiles-backend is running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(profile_service)
        .serve(addr)
        .await?;

    Ok(())
}
