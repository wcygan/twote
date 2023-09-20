use tonic::{Code, Request, Response, Status};

use schemas::login::login_service_server::LoginService;
use schemas::login::{LoginRequest, LoginResponse};

pub struct LoginServiceImpl;

#[tonic::async_trait]
impl LoginService for LoginServiceImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let message = format!(
            "oops! not implemented! Sorry {}!",
            request.into_inner().username
        );
        Err(Status::new(Code::Aborted, message))
    }
}
