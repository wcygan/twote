use tonic::{Code, Request, Response, Status};

use schemas::login::login_service_client::LoginServiceClient;
use schemas::login::login_service_server::LoginService;
use schemas::login::{LoginRequest, LoginResponse};

pub struct LoginServiceImpl;

#[tonic::async_trait]
impl LoginService for LoginServiceImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        // TODO: add a LoginServiceImpl constructor that creates a LoginServiceClient
        match LoginServiceClient::connect("http://localhost:50052").await {
            Ok(mut client) => client.login(request).await,
            Err(err) => Err(Status::new(Code::Internal, err.to_string())),
        }
    }
}
