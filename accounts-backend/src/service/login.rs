use tonic::{Code, Request, Response, Status};
use tracing::info;

use schemas::login::login_service_server::LoginService;
use schemas::login::{LoginRequest, LoginResponse};

pub struct LoginServiceImpl;

#[tonic::async_trait]
impl LoginService for LoginServiceImpl {
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
