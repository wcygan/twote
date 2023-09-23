use schemas::account::account_service_server::AccountService;
use schemas::account::CreateAccountRequest;
use schemas::account::CreateAccountResponse;
use schemas::account::{LoginRequest, LoginResponse};
use tonic::{Code, Request, Response, Status};
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
        let message = format!(
            "oops! not implemented! Sorry {}!",
            request.into_inner().username
        );
        Err(Status::new(Code::Aborted, message))
    }

    #[tracing::instrument(skip(self))]
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        info!("Processing CreateAccountRequest");
        let message = format!(
            "oops! not implemented! Sorry {}bot!",
            request.into_inner().username
        );
        Err(Status::new(Code::Aborted, message))
    }
}
