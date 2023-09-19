use tonic::transport::Server;

use schemas::hello::hello_service_server::HelloServiceServer;
use schemas::login::login_service_server::LoginServiceServer;

use crate::service::hello::HelloServiceImpl;
use crate::service::login::LoginServiceImpl;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    println!("Running on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(HelloServiceImpl))
        .add_service(LoginServiceServer::new(LoginServiceImpl::new().await?))
        .serve(addr)
        .await?;

    Ok(())
}
