use tonic::transport::Server;
use tracing::info;

use common::db::mongo::MongoDB;
use common::Service::TweetsBackend;
use schemas::tweet::tweet_service_server::TweetServiceServer;

use crate::service::tweet::TweetServiceImpl;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Initialize a MongoDB client from the Tweets db config
    let client = mongodb::Client::with_uri_str(MongoDB::Tweets.uri().as_str()).await?;

    // Create the services
    let (_, health_service) = tonic_health::server::health_reporter();
    let tweets_service = TweetServiceServer::new(TweetServiceImpl::new(client.clone()));

    // Start the server
    let addr = TweetsBackend.socket_addr();
    info!("tweets-backend is running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(tweets_service)
        .serve(addr)
        .await?;

    Ok(())
}
