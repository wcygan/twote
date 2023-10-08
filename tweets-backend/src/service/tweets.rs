use tonic::{Request, Response, Status};
use mongodb::bson;
use mongodb::bson::doc;
use std::time::Instant;
use tonic::codegen::tokio_stream::StreamExt;
use tracing::info;

// Import your generated proto modules and other necessary modules here.


use schemas::tweet::{
    Tweet, CreateTweetRequest, GetTweetRequest, FindTweetsByUser,
    BatchTweetRequest,BatchTweetResponse, FindMostRecentTweets,
};
use schemas::tweet::tweet_service_server::TweetService;

pub struct TweetServiceImpl {
    client: mongodb::Client,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TweetsDao  {
    _id: String,
    user_id: String,
    message: String,
    created_at: bson::Timestamp,
}

impl TweetServiceImpl {
    pub fn new(client: mongodb::Client) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl TweetService for TweetServiceImpl {
    async fn create(&self, request: Request<CreateTweetRequest>) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn get(&self, request: Request<GetTweetRequest>) -> Result<Response<Tweet>, Status> {
        unimplemented!()
    }

    async fn batch_get(&self, request: Request<BatchTweetRequest>) -> Result<Response<BatchTweetResponse>, Status> {
        unimplemented!()
    }

    async fn most_recent_tweets(&self, request: Request<FindMostRecentTweets>) -> Result<Response<BatchTweetResponse>, Status> {
        unimplemented!()
    }
}
