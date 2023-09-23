use anyhow::Result;
use common::Service::AccountsBackend;
use schemas::account::login_service_client::LoginServiceClient;
use schemas::account::login_service_server::LoginService;
use schemas::account::{LoginRequest, LoginResponse};
use tonic::{Request, Response, Status};
use tracing::info;

pub struct LoginServiceImpl;

#[tonic::async_trait]
impl LoginService for LoginServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("Processing LoginRequest");
        LoginServiceClient::connect(AccountsBackend.addr())
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?
            .login(request)
            .await
    }
}
