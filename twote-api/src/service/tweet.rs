use tonic::{Code, Request, Response, Status};
use tracing::info;

use common::Service::TweetsBackend;
use schemas::tweet::tweet_service_client::TweetServiceClient;
use schemas::tweet::tweet_service_server::TweetService;
use schemas::tweet::{
    BatchTweetRequest, BatchTweetResponse, CreateTweetRequest, FindMostRecentTweets,
    GetTweetRequest, Tweet,
};

pub struct TweetServiceImpl;

#[tonic::async_trait]
impl TweetService for TweetServiceImpl {
    async fn create(&self, request: Request<CreateTweetRequest>) -> Result<Response<()>, Status> {
        info!("Creating Tweet");
        TweetServiceClient::connect(TweetsBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .create(request)
            .await
    }

    async fn get(&self, request: Request<GetTweetRequest>) -> Result<Response<Tweet>, Status> {
        info!("Getting Tweet");
        TweetServiceClient::connect(TweetsBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .get(request)
            .await
    }

    async fn batch_get(
        &self,
        request: Request<BatchTweetRequest>,
    ) -> Result<Response<BatchTweetResponse>, Status> {
        info!("Batch-Get Tweets");
        TweetServiceClient::connect(TweetsBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .batch_get(request)
            .await
    }

    async fn most_recent_tweets(
        &self,
        request: Request<FindMostRecentTweets>,
    ) -> Result<Response<BatchTweetResponse>, Status> {
        info!("Get Most Recent Tweets");
        TweetServiceClient::connect(TweetsBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .most_recent_tweets(request)
            .await
    }
}
