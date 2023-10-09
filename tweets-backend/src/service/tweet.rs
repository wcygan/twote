use std::time::Instant;

use mongodb::bson;
use mongodb::bson::doc;
use tonic::{Request, Response, Status};
use tracing::info;
use uuid::Uuid;

use common::db::mongo::{collect_deserialize, MongoCollection, MongoDB};
use schemas::tweet::tweet_service_server::TweetService;
use schemas::tweet::{
    BatchTweetRequest, BatchTweetResponse, CreateTweetRequest, FindMostRecentTweetsByUserRequest,
    FindMostRecentTweetsRequest, GetTweetRequest, Tweet,
};

pub struct TweetServiceImpl {
    client: mongodb::Client,
}

impl TweetServiceImpl {
    pub fn new(client: mongodb::Client) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl TweetService for TweetServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn create(&self, request: Request<CreateTweetRequest>) -> Result<Response<()>, Status> {
        info!("Creating Tweet");

        // Insert the tweet into the database
        let tweet: bson::Document = TweetsDao::create_from(request.into_inner()).into();
        self.client
            .database(MongoDB::Tweets.name())
            .collection(MongoCollection::Tweets.name())
            .insert_one(tweet, None)
            .await
            .map_err(|_| Status::internal("Failed to create tweet"))?;

        Ok(Response::new(()))
    }
    #[tracing::instrument(skip(self))]
    async fn get(&self, request: Request<GetTweetRequest>) -> Result<Response<Tweet>, Status> {
        info!("Getting Tweet");

        let document = self
            .client
            .database(MongoDB::Tweets.name())
            .collection(MongoCollection::Tweets.name())
            .find_one(doc! { "_id": request.into_inner().tweet_id }, None)
            .await
            .map_err(|_| Status::internal("Failed to get tweet"))?
            .ok_or(Status::not_found("Tweet not found"))?;

        let tweet: TweetsDao =
            bson::from_document(document).map_err(|_| Status::internal("Failed to parse tweet"))?;

        Ok(Response::new(tweet.into()))
    }
    #[tracing::instrument(skip(self))]
    async fn batch_get(
        &self,
        _request: Request<BatchTweetRequest>,
    ) -> Result<Response<BatchTweetResponse>, Status> {
        info!("Batch-get Tweets");

        let tweet_id = _request.into_inner().tweet_ids;
        let cursor = self
            .client
            .database(MongoDB::Tweets.name())
            .collection(MongoCollection::Tweets.name())
            .find(
                doc! {
                    "_id": {
                        "$in": tweet_id,
                    },
                },
                None,
            )
            .await
            .map_err(|_| Status::internal("Failed to batch get tweets"))?;

        let tweets = collect_deserialize::<TweetsDao>(cursor, None)
            .await?
            .into_iter()
            .map(|tweet_dao| tweet_dao.into())
            .collect::<Vec<Tweet>>();

        Ok(Response::new(BatchTweetResponse { tweets }))
    }

    #[tracing::instrument(skip(self))]
    async fn find_most_recent_tweets(
        &self,
        _request: Request<FindMostRecentTweetsRequest>,
    ) -> Result<Response<BatchTweetResponse>, Status> {
        info!("Find Most Recent Tweets");

        let cursor = self
            .client
            .database(MongoDB::Tweets.name())
            .collection(MongoCollection::Tweets.name())
            .find(None, None)
            .await
            .map_err(|_| Status::internal("Failed to get all tweets"))?;

        let tweets = collect_deserialize::<TweetsDao>(cursor, None)
            .await?
            .into_iter()
            .map(|tweet_dao| tweet_dao.into())
            .collect::<Vec<Tweet>>();

        Ok(Response::new(BatchTweetResponse { tweets }))
    }

    #[tracing::instrument(skip(self))]
    async fn find_most_recent_tweets_by_user(
        &self,
        _request: Request<FindMostRecentTweetsByUserRequest>,
    ) -> Result<Response<BatchTweetResponse>, Status> {
        info!("Find Most Recent Tweets By User");

        let user_id = _request.into_inner().user_id;
        let cursor = self
            .client
            .database(MongoDB::Tweets.name())
            .collection(MongoCollection::Tweets.name())
            .find(
                doc! {
                    "user_id": user_id,
                },
                None,
            )
            .await
            .map_err(|_| Status::internal("Failed to get all tweets by user"))?;

        let tweets = collect_deserialize::<TweetsDao>(cursor, None)
            .await?
            .into_iter()
            .map(|tweet_dao| tweet_dao.into())
            .collect::<Vec<Tweet>>();

        // Build and return the response
        Ok(Response::new(BatchTweetResponse { tweets }))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TweetsDao {
    _id: String,
    user_id: String,
    message: String,
    created_at: bson::Timestamp,
    parent_tweet_id: Option<String>,
}

impl From<TweetsDao> for Tweet {
    fn from(val: TweetsDao) -> Self {
        let ts = prost_types::Timestamp {
            seconds: val.created_at.time as i64,
            nanos: 0,
        };

        Tweet {
            tweet_id: val._id,
            user_id: val.user_id,
            message: val.message,
            created_at: Some(ts),
            parent_tweet_id: val.parent_tweet_id.unwrap_or("".to_string()),
        }
    }
}

impl From<TweetsDao> for bson::Document {
    fn from(val: TweetsDao) -> Self {
        doc! {
            "_id": &val._id,
            "user_id": &val.user_id,
            "message": &val.message,
            "created_at": &val.created_at,
            "parent_tweet_id": &val.parent_tweet_id,
        }
    }
}

impl TweetsDao {
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
            parent_tweet_id: None,
        }
    }
}
