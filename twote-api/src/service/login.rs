use anyhow::Result;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};
use tub::Pool;

use schemas::login::login_service_client::LoginServiceClient;
use schemas::login::login_service_server::LoginService;
use schemas::login::{LoginRequest, LoginResponse};

pub struct LoginServiceImpl {
    login_service_clients: Pool<LoginServiceClient<Channel>>,
}

impl LoginServiceImpl {
    pub async fn new() -> Result<Self> {
        let client = LoginServiceClient::connect("http://localhost:50052").await?;

        Ok(LoginServiceImpl {
            login_service_clients: Pool::from_initializer(10, || client.clone()),
        })
    }
}

#[tonic::async_trait]
impl LoginService for LoginServiceImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let mut login_service_client = self.login_service_clients.acquire().await;
        login_service_client.login(request).await
    }
}
