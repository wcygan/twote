use tonic::{Code, Request, Response, Status};

use schemas::login::login_service_server::LoginService;
use schemas::login::{LoginRequest, LoginResponse};

pub struct LoginServiceImpl;

#[tonic::async_trait]
impl LoginService for LoginServiceImpl {
    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let message = format!("not implemented. Sorry {}!", _request.into_inner().username);
        Err(Status::new(Code::Aborted, message))
    }
}
