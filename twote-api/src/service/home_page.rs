use common::helpers::tweet_details;
use common::Service::{ProfilesBackend, TweetsBackend};
use schemas::frontend::home_page_service_server::HomePageService;
use schemas::frontend::HomePage;
use schemas::profile::profile_service_client::ProfileServiceClient;
use schemas::profile::{BatchGetProfileRequest, FindMostRecentProfilesRequest, Profile};
use schemas::tweet::tweet_service_client::TweetServiceClient;
use schemas::tweet::{FindMostRecentTweetsRequest, Tweet};
use std::collections::{HashMap, HashSet};
use tonic::{Code, Request, Response, Status};
use tracing::info;

pub struct HomePageServiceImpl;

#[tonic::async_trait]
impl HomePageService for HomePageServiceImpl {
    async fn get_home_page(&self, _request: Request<()>) -> Result<Response<HomePage>, Status> {
        let tweets = self.find_most_recent_tweets();
        let profiles = self.find_most_recent_profiles();

        let (tweets, recent_profiles) = tokio::join!(tweets, profiles);
        let tweets = tweets?;

        let unique_tweet_profiles: Vec<String> = tweets
            .iter()
            .map(|tweet| tweet.user_id.clone())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        let user_id_to_profile: HashMap<String, Profile> = self
            .batch_get_profiles(unique_tweet_profiles)
            .await?
            .into_iter()
            .map(|profile| (profile.user_id.clone(), profile))
            .collect();

        let tweets = tweets
            .into_iter()
            .map(|tweet| {
                let profile = user_id_to_profile
                    .get(&tweet.user_id)
                    .ok_or(Status::new(Code::Internal, "Profile not found"));
                Ok::<schemas::frontend::Tweet, Status>(tweet_details(tweet, profile?))
            })
            .collect::<Result<Vec<schemas::frontend::Tweet>, Status>>()?;

        let home_page = HomePage {
            tweets,
            profiles: recent_profiles?,
        };

        Ok(Response::new(home_page))
    }
}

impl HomePageServiceImpl {
    async fn find_most_recent_tweets(&self) -> Result<Vec<Tweet>, Status> {
        info!("Getting Tweets by Profile");
        TweetServiceClient::connect(TweetsBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .find_most_recent_tweets(FindMostRecentTweetsRequest {})
            .await
            .map(|response| response.into_inner().tweets)
    }

    async fn find_most_recent_profiles(&self) -> Result<Vec<Profile>, Status> {
        ProfileServiceClient::connect(ProfilesBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .find_most_recent_profiles(FindMostRecentProfilesRequest {})
            .await
            .map(|response| response.into_inner().profiles)
    }

    async fn batch_get_profiles(&self, user_ids: Vec<String>) -> Result<Vec<Profile>, Status> {
        ProfileServiceClient::connect(ProfilesBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .batch_get(BatchGetProfileRequest { user_ids })
            .await
            .map(|response| response.into_inner().profiles)
    }
}
