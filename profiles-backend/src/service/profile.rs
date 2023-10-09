use common::db::mongo::{collect_deserialize, MongoCollection, MongoDB};
use mongodb::bson;
use mongodb::bson::doc;
use std::time::Instant;
use tonic::{Request, Response, Status};
use tracing::info;

use schemas::profile::profile_service_server::ProfileService;
use schemas::profile::{
    BatchGetProfileRequest, BatchProfileResponse, CreateProfileRequest,
    FindMostRecentProfilesRequest, GetProfileRequest, Profile,
};

pub struct ProfileServiceImpl {
    client: mongodb::Client,
}

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn create(&self, request: Request<CreateProfileRequest>) -> Result<Response<()>, Status> {
        info!("Creating Profile");

        // Insert the profile into the database
        let bson_data: bson::Document = ProfileDao::create_from(request.into_inner()).into();
        self.client
            .database(MongoDB::Profiles.name())
            .collection(MongoCollection::Profiles.name())
            .insert_one(bson_data, None)
            .await
            .map_err(|_| Status::internal("Failed to create profile"))?;

        Ok(Response::new(()))
    }

    #[tracing::instrument(skip(self))]
    async fn get(&self, _request: Request<GetProfileRequest>) -> Result<Response<Profile>, Status> {
        info!("Getting Profile");

        // Get the profile from the database
        let document = self
            .client
            .database(MongoDB::Profiles.name())
            .collection(MongoCollection::Profiles.name())
            .find_one(
                doc! {
                    "_id": _request.into_inner().user_id,
                },
                None,
            )
            .await
            .map_err(|_| Status::internal("Failed to get profile"))?
            .ok_or(Status::not_found("Profile not found"))?;

        let profile: ProfileDao = bson::from_document(document)
            .map_err(|_| Status::internal("Failed to parse profile"))?;

        Ok(Response::new(profile.into()))
    }

    #[tracing::instrument(skip(self))]
    async fn batch_get(
        &self,
        _request: Request<BatchGetProfileRequest>,
    ) -> Result<Response<BatchProfileResponse>, Status> {
        info!("Batch-Get Profiles");

        // Get the profiles from the database
        let user_ids = _request.into_inner().user_ids;
        let cursor = self
            .client
            .database(MongoDB::Profiles.name())
            .collection(MongoCollection::Profiles.name())
            .find(
                doc! {
                    "_id": {
                        "$in": user_ids,
                    },
                },
                None,
            )
            .await
            .map_err(|_| Status::internal("Failed to get profiles"))?;

        let profiles = collect_deserialize::<ProfileDao>(cursor, None)
            .await?
            .into_iter()
            .map(|tweet_dao| tweet_dao.into())
            .collect::<Vec<Profile>>();

        Ok(Response::new(BatchProfileResponse { profiles }))
    }

    #[tracing::instrument(skip(self))]
    async fn find_most_recent_profiles(
        &self,
        _request: Request<FindMostRecentProfilesRequest>,
    ) -> Result<Response<BatchProfileResponse>, Status> {
        let cursor = self
            .client
            .database(MongoDB::Profiles.name())
            .collection(MongoCollection::Profiles.name())
            .find(None, None)
            .await
            .map_err(|_| Status::internal("Failed to get profiles"))?;

        let profiles = collect_deserialize::<ProfileDao>(cursor, None)
            .await?
            .into_iter()
            .map(|tweet_dao| tweet_dao.into())
            .collect::<Vec<Profile>>();

        Ok(Response::new(BatchProfileResponse { profiles }))
    }
}

impl ProfileServiceImpl {
    pub fn new(client: mongodb::Client) -> Self {
        Self { client }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ProfileDao {
    _id: String,
    first_name: String,
    last_name: String,
    bio: String,
    joined_at: bson::Timestamp,
}

impl From<ProfileDao> for Profile {
    fn from(val: ProfileDao) -> Self {
        let ts = prost_types::Timestamp {
            seconds: val.joined_at.time as i64,
            nanos: 0,
        };

        Profile {
            user_id: val._id,
            first_name: val.first_name,
            last_name: val.last_name,
            biography: val.bio,
            joined_at: Some(ts),
        }
    }
}

impl From<ProfileDao> for bson::Document {
    fn from(val: ProfileDao) -> Self {
        doc! {
            "_id": &val._id,
            "first_name": &val.first_name,
            "last_name": &val.last_name,
            "bio": &val.bio,
            "joined_at": &val.joined_at,
        }
    }
}

impl ProfileDao {
    fn create_from(request: CreateProfileRequest) -> Self {
        let ts = bson::Timestamp {
            time: Instant::now().elapsed().as_secs() as u32,
            increment: 0,
        };

        Self {
            _id: request.user_id,
            first_name: request.first_name,
            last_name: request.last_name,
            bio: String::new(),
            joined_at: ts,
        }
    }
}
