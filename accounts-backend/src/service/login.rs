use tonic::{Code, Request, Response, Status};
use tracing::info;

use schemas::account::account_service_server::AccountService;
use schemas::account::{LoginRequest, LoginResponse};

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
}
