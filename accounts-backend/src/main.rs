use crate::service::login::LoginServiceImpl;
use schemas::login::login_service_server::LoginServiceServer;
use tonic::transport::Server;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50052".parse()?;

    println!("Running on {}", addr);

    Server::builder()
        .add_service(LoginServiceServer::new(LoginServiceImpl))
        .serve(addr)
        .await?;

    Ok(())
}
