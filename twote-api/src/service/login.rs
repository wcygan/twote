use anyhow::Result;
use common::Service::AccountsBackend;
use schemas::account::account_service_client::AccountServiceClient;
use schemas::account::account_service_server::AccountService;
use schemas::account::{LoginRequest, LoginResponse};
use tonic::{Request, Response, Status};
use tracing::info;

pub struct AccountServiceImpl;

#[tonic::async_trait]
impl AccountService for AccountServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("Processing LoginRequest");
        AccountServiceClient::connect(AccountsBackend.addr())
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?
            .login(request)
            .await
    }
}
