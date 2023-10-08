use tonic::{Request, Response, Status};
use mongodb::bson;
use mongodb::bson::doc;
use std::time::Instant;
use tonic::codegen::tokio_stream::StreamExt;
use tracing::info;
use uuid::Uuid;
use common::{MongoCollection, MongoDB};

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
        info!("Creating Profile");

        // Insert the profile into the database
        let bson_data = TweetsDao::create_from(request.into_inner()).to_bson();
        self.client
            .database(MongoDB::Tweets.name())
            .collection(MongoCollection::Tweets.name())
            .insert_one(bson_data, None)
            .await
            .map_err(|_| Status::internal("Failed to create profile"))?;

        Ok(Response::new(()))
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

impl TweetsDao{
    fn create_from(request: CreateTweetRequest) -> Self {
        let created_at = bson::Timestamp {
            time: Instant::now().elapsed().as_secs() as u32,
            increment: 0,
        };

        Self {
            _id: Uuid::new_v4().to_string(),
            user_id: request.user_id,
            message: request.message,
            created_at,
        }
    }
    fn as_proto(self) -> Tweet {
        let ts = prost_types::Timestamp {
            seconds: self.created_at.time as i64,
            nanos: 0,
        };

        Tweet {
            tweet_id: self._id,
            user_id: self.user_id,
            message: self.message,
            created_at: Some(ts),
        }
    }

    fn to_bson(&self) -> bson::Document {
        doc! {
            "_id": &self._id,
            "user_id": &self.user_id,
            "message": &self.message,
            "created_at": &self.created_at,
        }
    }
}


