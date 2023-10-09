use common::helpers::tweet_details;
use common::Service::{ProfilesBackend, TweetsBackend};
use schemas::frontend::profile_page_service_server::ProfilePageService;
use schemas::frontend::{GetProfilePageRequest, ProfilePage};
use schemas::profile::profile_service_client::ProfileServiceClient;
use schemas::profile::{GetProfileRequest, Profile};
use schemas::tweet::tweet_service_client::TweetServiceClient;
use schemas::tweet::{FindMostRecentTweetsByUserRequest, Tweet};
use tonic::{Code, Request, Response, Status};
use tracing::info;

pub struct ProfilePageServiceImpl;

#[tonic::async_trait]
impl ProfilePageService for ProfilePageServiceImpl {
    async fn get_profile_page(
        &self,
        request: Request<GetProfilePageRequest>,
    ) -> Result<Response<ProfilePage>, Status> {
        let user_id = request.into_inner().user_id;
        let profile = self.get_profile(user_id.clone()).await?;
        let tweets = self
            .find_most_recent_tweets_by_profile(user_id)
            .await?
            .into_iter()
            .map(|tweet| tweet_details(tweet, &profile))
            .collect();

        let profile_page = ProfilePage {
            user_id: profile.user_id,
            first_name: profile.first_name,
            last_name: profile.last_name,
            biography: profile.biography,
            joined_at: profile.joined_at,
            tweets,
        };

        Ok(Response::new(profile_page))
    }
}

impl ProfilePageServiceImpl {
    async fn get_profile(&self, user_id: String) -> Result<Profile, Status> {
        info!("Getting Profile");
        ProfileServiceClient::connect(ProfilesBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .get(GetProfileRequest { user_id })
            .await
            .map(|response| response.into_inner())
    }

    async fn find_most_recent_tweets_by_profile(
        &self,
        user_id: String,
    ) -> Result<Vec<Tweet>, Status> {
        info!("Getting Tweets by Profile");
        TweetServiceClient::connect(TweetsBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .find_most_recent_tweets_by_user(FindMostRecentTweetsByUserRequest { user_id })
            .await
            .map(|response| response.into_inner().tweets)
    }
}
