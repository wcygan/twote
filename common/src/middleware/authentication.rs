use std::task::{Context, Poll};

use hyper::Body;
use redis::AsyncCommands;
use tonic::body::BoxBody;
use tonic::{Code, Status};
use tower::{Layer, Service};
use tracing::info;

const ALLOWED_UNAUTHORIZED_PATHS: [&str; 2] = [
    "/account.AccountService/Login",
    "/account.AccountService/CreateAccount",
];

#[derive(Default, Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> AuthMiddleware<S> {
    pub fn new(service: S) -> Self {
        Self { inner: service }
    }
}

impl<S> Layer<S> for AuthMiddleware<S> {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

impl<S> Service<hyper::Request<Body>> for AuthMiddleware<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // TODO: figure out where the token can come from
        info!("checking for token {:?}", req);

        Box::pin(async move {
            if ALLOWED_UNAUTHORIZED_PATHS.contains(&req.uri().path()) {
                return inner.call(req).await;
            }

            if let Some(Ok(token)) = req.headers().get("authorization").map(|v| v.to_str()) {
                info!("got token {:?}", token);

                if let Ok(value) = get_token(token.to_string()).await {
                    if let Ok(v) = value.parse() {
                        // TODO: verify that this is working (or fix it)
                        req.headers_mut().insert("user-id", v);
                    }

                    return inner.call(req).await;
                }
            }

            Ok(Status::unauthenticated("please sign in first").to_http())
        })
    }
}

// TODO: move this into `impl<S> AuthMiddleware<S>` and initiate a pool of redis clients to reuse
//       Make it `Clone` so that you can clone before Box::pin
async fn get_token(key: String) -> Result<String, Status> {
    let client = redis::Client::open("redis://token-cache/")
        .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

    let mut con = client
        .get_async_connection()
        .await
        .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

    con.get(key)
        .await
        .map_err(|e| Status::new(Code::Internal, e.to_string()))
}
