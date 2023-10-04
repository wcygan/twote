use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use redis::AsyncCommands;
use sqlx::PgPool;
use tonic::{Code, Request, Response, Status};
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use common::spawn_blocking_with_tracing;
use common::Service::ProfilesBackend;
use schemas::account::account_service_server::AccountService;
use schemas::account::CreateAccountRequest;
use schemas::account::CreateAccountResponse;
use schemas::account::{LoginRequest, LoginResponse};
use schemas::profile::profile_service_client::ProfileServiceClient;
use schemas::profile::CreateProfileRequest;

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
        let req = request.into_inner();

        let data = LoginData {
            username: req.username,
            password: req.password,
        };

        data.validate()
            .map_err(|e| Status::new(Code::InvalidArgument, e.to_string()))?;

        self.validate_credentials(data).await
    }

    #[tracing::instrument(skip(self))]
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        info!("Processing CreateAccountRequest");
        let request = request.into_inner();
        let data = create_account_data(&request).await?;

        // TODO: use a transaction and commit only if profile creation succeeds
        match self.persist_credentials(data).await {
            Ok(result) => {
                let create_profile_request = CreateProfileRequest {
                    user_id: result.user_id.clone(),
                    first_name: request.first_name,
                    last_name: request.last_name,
                };

                ProfileServiceClient::connect(ProfilesBackend.addr())
                    .await
                    .map_err(|e| Status::new(Code::Internal, e.to_string()))?
                    .create(Request::new(create_profile_request))
                    .await?;

                Ok(Response::new(result))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

impl AccountServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "Persist credentials", skip(self))]
    async fn persist_credentials(&self, data: Account) -> Result<CreateAccountResponse, Status> {
        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)",
            data.user_id,
            data.username,
            data.password_hash,
        )
        .execute(&self.pool)
        .await
        .map(|_| CreateAccountResponse {
            user_id: data.user_id.to_string(),
        })
        .map_err(|e| Status::new(Code::AlreadyExists, e.to_string()))
    }

    #[tracing::instrument(name = "Validate credentials", skip(self))]
    async fn validate_credentials(
        &self,
        data: LoginData,
    ) -> Result<Response<LoginResponse>, Status> {
        let (user_id, stored_password_hash) = self.get_credentials(&data).await?;

        verify_password_hash(stored_password_hash, data.password)
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        let token = Uuid::new_v4().to_string();
        self.persist_token(token.clone(), user_id.to_string())
            .await?;

        Ok(Response::new(LoginResponse { token }))
    }

    #[tracing::instrument(name = "Get credentials", skip(self))]
    async fn get_credentials(&self, data: &LoginData) -> Result<(Uuid, String), Status> {
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

    // TODO: initiate a pool of redis clients to reuse
    #[tracing::instrument(name = "Persisting token to redis", skip(self))]
    async fn persist_token(&self, key: String, value: String) -> Result<(), Status> {
        info!("persisting ({key}, {value})");

        let client = redis::Client::open("redis://token-cache/")
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        let mut con = client
            .get_async_connection()
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        con.set(key, value)
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))
    }
}

#[tracing::instrument(name = "Create account data", skip(request))]
async fn create_account_data(request: &CreateAccountRequest) -> Result<Account, Status> {
    // Validate the credentials provided by the user
    let credentials = validate_new_credentials(request)?;

    // Hash the password with salt
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = compute_password_hash(credentials.password, salt)
        .await
        .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

    // Create the data to be stored in the database
    let account = Account {
        user_id: Uuid::new_v4(),
        username: credentials.username,
        password_hash,
    };

    Ok(account)
}

#[tracing::instrument(name = "Validate credentials", skip(request))]
fn validate_new_credentials(request: &CreateAccountRequest) -> Result<LoginData, Status> {
    let u = LoginData {
        username: request.username.clone(),
        password: request.password.clone(),
    };

    u.validate()
        .map_err(|e| Status::new(Code::InvalidArgument, e.to_string()))?;

    Ok(u)
}

#[tracing::instrument(name = "Compute password hash", skip(password, salt))]
async fn compute_password_hash(
    password: String,
    salt: SaltString,
) -> Result<String, anyhow::Error> {
    spawn_blocking_with_tracing(move || {
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
) -> Result<(), anyhow::Error> {
    spawn_blocking_with_tracing(move || {
        let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())
            .context("Failed to parse hash in PHC string format.")?;

        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected_password_hash)
            .context("Invalid password.")
    })
    .await?
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn password_hash_computation() {
        let password = "password".to_string();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let _salt_string = salt.to_string();
        let password_hash = compute_password_hash(password.clone(), salt).await.unwrap();
        assert_ne!(password_hash, password);
    }

    #[tokio::test]
    async fn password_verification_success() {
        let password = "password".to_string();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let _salt_string = salt.to_string();
        let password_hash = compute_password_hash(password.clone(), salt).await.unwrap();
        let result = verify_password_hash(password_hash, password).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn password_verification_failure() {
        let password = "password".to_string();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let _salt_string = salt.to_string();
        let password_hash = compute_password_hash(password.clone(), salt).await.unwrap();
        let result = verify_password_hash(password_hash, "foobar".to_string()).await;
        assert!(result.is_err());
    }
}
