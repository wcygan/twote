use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use common::spawn_blocking_with_tracing;
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
        let (user_id, stored_password_hash, stored_salt) = self.get_credentials(data).await?;

        // TODO @wcygan: maybe there is a bug here since we're not using the salt
        // spawn_blocking_with_tracing(move || {
        //     verify_password_hash(expected_password_hash, credentials.password)
        // })
        //     .await
        //     .context("Failed to spawn blocking task.")
        //     .map_err(AuthError::UnexpectedError)??;
        //
        // user_id
        //     .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        //     .map_err(AuthError::InvalidCredentials)

        unimplemented!()
    }

    #[tracing::instrument(name = "Get credentials", skip(self))]
    async fn get_credentials(&self, data: LoginData) -> Result<(Uuid, String, String), Status> {
        sqlx::query!(
            r#"
            SELECT user_id, password_hash, salt
            FROM users
            WHERE username = $1
            "#,
            data.username,
        )
        .fetch_one(&self.pool)
        .await
        .map_or(
            Err(Status::new(Code::Internal, "Credentials not found")),
            |row| Ok((row.user_id, row.password_hash, row.salt)),
        )
    }
}

#[tracing::instrument(name = "Create account data", skip(request))]
fn create_account_data(request: Request<CreateAccountRequest>) -> Result<Account, Status> {
    // Validate the credentials provided by the user
    let credentials = validate_new_credentials(request.into_inner())?;

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

#[tracing::instrument(name = "Validate credentials", skip(request))]
fn validate_new_credentials(request: CreateAccountRequest) -> Result<LoginData, Status> {
    let u = LoginData {
        username: request.username,
        password: request.password,
    };

    u.validate()
        .map_err(|e| Status::new(Code::InvalidArgument, e.to_string()))?;

    Ok(u)
}

#[tracing::instrument(name = "Compute password hash", skip(password, salt))]
async fn compute_password_hash(password: String, salt: String) -> Result<String, anyhow::Error> {
    spawn_blocking_with_tracing(move || {
        let salt = SaltString::encode_b64(&salt.as_bytes())?;
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
        Ok(password_hash)
    })
    .await?
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), AuthError> {
    spawn_blocking_with_tracing(move || {
        let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())
            .context("Failed to parse hash in PHC string format.")
            .map_err(AuthError::UnexpectedError)?;

        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected_password_hash)
            .context("Invalid password.")
            .map_err(AuthError::InvalidCredentials)
    })
    .await?
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
}
