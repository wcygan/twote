use common::{MongoCollection, MongoDB};
use mongodb::bson;
use mongodb::bson::doc;
use std::time::Instant;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::{Request, Response, Status};
use tracing::info;

use schemas::profile::profile_service_server::ProfileService;
use schemas::profile::{
    BatchGetProfileRequest, BatchGetProfileResponse, CreateProfileRequest, GetProfileRequest,
    Profile,
};

pub struct ProfileServiceImpl {
    client: mongodb::Client,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ProfileDao {
    _id: String,
    first_name: String,
    last_name: String,
    bio: String,
    joined_at: bson::Timestamp,
}

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn create(
        &self,
        _request: Request<CreateProfileRequest>,
    ) -> Result<Response<()>, Status> {
        info!("Creating Profile");

        // Insert the profile into the database
        let bson_data = ProfileDao::create_from(_request.into_inner()).to_bson();
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
        let profile = self
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
            .map_err(|_| Status::internal("Failed to get profile"))?;

        // Build and return the response
        match profile {
            Some(profile) => {
                info!("Found profile: {:?}", profile);
                let profile_dao = bson::from_document::<ProfileDao>(profile).map_err(|e| {
                    info!("Failed to deserialize profile: {:?}", e);
                    Status::internal("Failed to get profile")
                })?;
                Ok(Response::new(profile_dao.as_proto()))
            }
            None => Err(Status::not_found("Profile not found")),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn batch_get(
        &self,
        _request: Request<BatchGetProfileRequest>,
    ) -> Result<Response<BatchGetProfileResponse>, Status> {
        info!("Batch-Get Profiles");

        // Get the profiles from the database
        let user_ids = _request.into_inner().user_ids;
        let mut cursor = self
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

        // Collect the resulting profiles into a vector
        let mut profiles = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => match bson::from_document::<ProfileDao>(document) {
                    Ok(profile_dao) => profiles.push(profile_dao.as_proto()),
                    Err(_) => return Err(Status::internal("Failed to deserialize profile")),
                },
                Err(_) => return Err(Status::internal("Failed to get profile")),
            }
        }

        // Build and return the response
        Ok(Response::new(BatchGetProfileResponse { profiles }))
    }
}

impl ProfileServiceImpl {
    pub fn new(client: mongodb::Client) -> Self {
        Self { client }
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

    fn to_bson(&self) -> bson::Document {
        doc! {
            "_id": &self._id,
            "first_name": &self.first_name,
            "last_name": &self.last_name,
            "bio": &self.bio,
            "joined_at": &self.joined_at,
        }
    }

    fn as_proto(&self) -> Profile {
        let ts = prost_types::Timestamp {
            seconds: self.joined_at.time as i64,
            nanos: 0,
        };

        Profile {
            user_id: self._id.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            biography: self.bio.clone(),
            joined_at: Some(ts),
        }
    }
}
