use crate::service::login::AccountServiceImpl;
use common::Service::AccountsBackend;
use schemas::account::account_service_server::AccountServiceServer;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tonic::transport::Server;
use tracing::info;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create a database connection pool
    let pool = PgPoolOptions::new()
        .connect(env::var("DATABASE_URL").unwrap().as_str())
        .await?;

    // Apply the migrations
    sqlx::migrate!().run(&pool).await?;

    // Create the services
    let login_service = AccountServiceServer::new(AccountServiceImpl::new(pool.clone()));
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    let addr = AccountsBackend.socket_addr();
    info!("accounts-backend is running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(login_service)
        .serve(addr)
        .await?;

    Ok(())
}
