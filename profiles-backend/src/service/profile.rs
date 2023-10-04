use tonic::{Request, Response, Status};
use tracing::info;

use schemas::profile::profile_service_server::ProfileService;
use schemas::profile::{
    BatchGetProfileRequest, BatchGetProfileResponse, CreateProfileRequest, GetProfileRequest,
    Profile,
};

pub struct ProfileServiceImpl;

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn create(
        &self,
        _request: Request<CreateProfileRequest>,
    ) -> Result<Response<()>, Status> {
        info!("Creating Profile");
        // TODO: ensure the profiles-db database exists and the profiles collection exists
        // TODO: persist the profile to MongoDB
        Err(Status::unimplemented("create profile"))
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
