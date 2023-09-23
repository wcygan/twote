use anyhow::Result;
use common::Service::AccountsBackend;
use schemas::account::account_service_client::AccountServiceClient;
use schemas::account::account_service_server::AccountService;
use schemas::account::CreateAccountRequest;
use schemas::account::CreateAccountResponse;
use schemas::account::{LoginRequest, LoginResponse};
use tonic::Code;
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

    #[tracing::instrument(skip(self))]
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        info!("Processing LoginRequest");
        let message = format!(
            "oops! not implemented! Sorry {}!",
            request.into_inner().username
        );
        Err(Status::new(Code::Aborted, message))
    }
}
