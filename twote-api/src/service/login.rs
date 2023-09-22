use anyhow::Result;
use common::Service::AccountsBackend;
use schemas::login::login_service_client::LoginServiceClient;
use schemas::login::login_service_server::LoginService;
use schemas::login::{LoginRequest, LoginResponse};
use tonic::{Request, Response, Status};

pub struct LoginServiceImpl;

#[tonic::async_trait]
impl LoginService for LoginServiceImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        LoginServiceClient::connect(AccountsBackend.addr())
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?
            .login(request)
            .await
    }
}
