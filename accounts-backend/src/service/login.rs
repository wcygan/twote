use schemas::account::account_service_server::AccountService;
use schemas::account::CreateAccountRequest;
use schemas::account::CreateAccountResponse;
use schemas::account::{LoginRequest, LoginResponse};
use sqlx::PgPool;
use tonic::{Code, Request, Response, Status};
use tracing::info;
use uuid::Uuid;

pub struct AccountServiceImpl {
    pool: PgPool,
}

impl AccountServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn try_create(
        &self,
        req: CreateAccountRequest,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        sqlx::query!(
            "INSERT INTO users (user_id, username, password)
            VALUES ($1, $2, $3)",
            Uuid::new_v4(),
            req.username,
            req.password,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Status::new(Code::AlreadyExists, e.to_string()))?;
        let message = format!("oops! not implemented! Sorrybot!");
        Err(Status::new(Code::Aborted, message))
    }
}

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
        self.try_create(request.into_inner()).await
    }
}
