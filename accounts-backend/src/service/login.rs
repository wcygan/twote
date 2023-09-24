use schemas::account::account_service_server::AccountService;
use schemas::account::CreateAccountRequest;
use schemas::account::CreateAccountResponse;
use schemas::account::{LoginRequest, LoginResponse};
use sqlx::PgPool;
use tonic::{Code, Request, Response, Status};
use tracing::info;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate)]
struct SignupData {
    #[validate(length(min = 4))]
    username: String,
    #[validate(length(min = 4))]
    password: String,
}

pub struct AccountServiceImpl {
    pool: PgPool,
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
        let data = validate_credentials(request.into_inner())?;
        self.create_account(data).await
    }
}

impl AccountServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn create_account(
        &self,
        data: SignupData,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        sqlx::query!(
            "INSERT INTO users (user_id, username, password)
            VALUES ($1, $2, $3)",
            Uuid::new_v4(),
            data.username,
            data.password,
        )
        .execute(&self.pool)
        .await
        .map(|_| Response::new(CreateAccountResponse {}))
        .map_err(|e| Status::new(Code::AlreadyExists, e.to_string()))
    }
}

fn validate_credentials(req: CreateAccountRequest) -> Result<SignupData, Status> {
    let u = SignupData {
        username: req.username,
        password: req.password,
    };

    u.validate()
        .map_err(|e| Status::new(Code::InvalidArgument, e.to_string()))?;

    Ok(u)
}
