use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
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
struct LoginData {
    #[validate(length(min = 4))]
    username: String,
    #[validate(length(min = 4))]
    password: String,
}

#[derive(Debug)]
struct Account {
    user_id: Uuid,
    username: String,
    password_hash: String,
    salt: String,
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
        let data = create_account_data(request)?;
        self.persist_credentials(data).await
    }
}

impl AccountServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "Persist credentials", skip(self))]
    async fn persist_credentials(
        &self,
        data: Account,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash, salt)
            VALUES ($1, $2, $3, $4)",
            data.user_id,
            data.username,
            data.password_hash,
            data.salt,
        )
        .execute(&self.pool)
        .await
        .map(|_| Response::new(CreateAccountResponse {}))
        .map_err(|e| Status::new(Code::AlreadyExists, e.to_string()))
    }

    #[tracing::instrument(name = "Validate credentials", skip(self))]
    async fn validate_credentials(
        &self,
        data: LoginData,
    ) -> Result<Option<(Uuid, String)>, Status> {
        let (user_id, password_hash) = self.get_credentials(data).await?;

        unimplemented!()
    }

    #[tracing::instrument(name = "Get credentials", skip(self))]
    async fn get_credentials(&self, data: LoginData) -> Result<(Uuid, String), Status> {
        sqlx::query!(
            r#"
            SELECT user_id, password_hash
            FROM users
            WHERE username = $1
            "#,
            data.username,
        )
        .fetch_one(&self.pool)
        .await
        .map_or(
            Err(Status::new(Code::Internal, "Credentials not found")),
            |row| Ok((row.user_id, row.password_hash)),
        )
    }
}

fn create_account_data(request: Request<CreateAccountRequest>) -> Result<Account, Status> {
    // Validate the credentials provided by the user
    let credentials = validate_credentials(request.into_inner())?;

    // Hash the password with salt
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(credentials.password.as_bytes(), &salt)
    .unwrap()
    .to_string();

    // Create the data to be stored in the database
    let account = Account {
        user_id: Uuid::new_v4(),
        username: credentials.username,
        password_hash,
        salt: salt.to_string(),
    };

    Ok(account)
}

fn validate_credentials(request: CreateAccountRequest) -> Result<LoginData, Status> {
    let u = LoginData {
        username: request.username,
        password: request.password,
    };

    u.validate()
        .map_err(|e| Status::new(Code::InvalidArgument, e.to_string()))?;

    Ok(u)
}
