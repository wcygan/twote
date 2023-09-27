use tonic::{Request, Response, Status};
use tracing::info;

use schemas::profile::profile_service_server::ProfileService;
use schemas::profile::{
    BatchGetProfileRequest, BatchGetProfileResponse, GetProfileRequest, Profile,
};

pub struct ProfileServiceImpl;

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn create(&self, _request: Request<Profile>) -> Result<Response<()>, Status> {
        info!("Creating Profile");
        todo!("create profiles in profiles-db")
    }

    #[tracing::instrument(skip(self))]
    async fn get(&self, _request: Request<GetProfileRequest>) -> Result<Response<Profile>, Status> {
        info!("Getting Profile");
        todo!("get profiles from profiles-db")
    }

    #[tracing::instrument(skip(self))]
    async fn batch_get(
        &self,
        _request: Request<BatchGetProfileRequest>,
    ) -> Result<Response<BatchGetProfileResponse>, Status> {
        info!("Batch-Get Profiles");
        todo!("batch-get profiles from profiles-db")
    }
}
