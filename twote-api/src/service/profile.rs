use common::Service::ProfilesBackend;
use schemas::profile::profile_service_client::ProfileServiceClient;
use schemas::profile::profile_service_server::ProfileService;
use schemas::profile::{
    BatchGetProfileRequest, BatchProfileResponse, CreateProfileRequest,
    FindMostRecentProfilesRequest, GetProfileRequest, Profile,
};
use tonic::{Code, Request, Response, Status};
use tracing::info;

pub struct ProfileServiceImpl;

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn create(&self, request: Request<CreateProfileRequest>) -> Result<Response<()>, Status> {
        info!("Creating Profile");
        ProfileServiceClient::connect(ProfilesBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .create(request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get(&self, request: Request<GetProfileRequest>) -> Result<Response<Profile>, Status> {
        info!("Getting Profile");
        ProfileServiceClient::connect(ProfilesBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .get(request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn batch_get(
        &self,
        request: Request<BatchGetProfileRequest>,
    ) -> Result<Response<BatchProfileResponse>, Status> {
        info!("Batch-Get Profiles");
        ProfileServiceClient::connect(ProfilesBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .batch_get(request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn find_most_recent_profiles(
        &self,
        request: Request<FindMostRecentProfilesRequest>,
    ) -> Result<Response<BatchProfileResponse>, Status> {
        info!("Get Random Profiles");
        ProfileServiceClient::connect(ProfilesBackend.addr())
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .find_most_recent_profiles(request)
            .await
    }
}
