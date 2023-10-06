use common::{MongoCollection, MongoDB};
use mongodb::bson;
use mongodb::bson::doc;
use std::time::Instant;
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

struct ProfileDao {
    id: String,
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
        Err(Status::unimplemented("get profile"))
    }

    #[tracing::instrument(skip(self))]
    async fn batch_get(
        &self,
        _request: Request<BatchGetProfileRequest>,
    ) -> Result<Response<BatchGetProfileResponse>, Status> {
        info!("Batch-Get Profiles");
        Err(Status::unimplemented("batch get profiles"))
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
            id: request.user_id,
            first_name: request.first_name,
            last_name: request.last_name,
            bio: String::new(),
            joined_at: ts,
        }
    }

    fn to_bson(&self) -> bson::Document {
        doc! {
            "_id": &self.id,
            "first_name": &self.first_name,
            "last_name": &self.last_name,
            "bio": &self.bio,
            "joined_at": &self.joined_at,
        }
    }

    // TODO: write a to_profile() function for the protobuf schema
}
