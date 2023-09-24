use hyper::Body;
use std::task::{Context, Poll};
use tonic::transport::Server;
use tonic::{Request, Status};
use tower::Service;
use tracing::info;

#[tracing::instrument()]
pub fn auth_interceptor(request: Request<()>) -> Result<Request<()>, Status> {
    if valid_credentials(&request) {
        Ok(request)
    } else {
        Err(Status::unauthenticated("invalid credentials"))
    }
}

fn valid_credentials(request: &Request<()>) -> bool {
    true
}

#[derive(Default)]
struct AuthMiddleware<S> {
    inner: S,
}

impl<S> AuthMiddleware<S> {
    fn new(service: S) -> Self {
        Self { inner: service }
    }
}

impl<S> Service<hyper::Request<Body>> for AuthMiddleware<S>
where
    S: Service<hyper::Request<Body>>,
    S::Error: Into<Status>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[tracing::instrument(skip(self))]
    fn call(&mut self, req: hyper::Request<Body>) -> Self::Future {
        // Get the URI from the request.
        let uri = req.uri();

        // Perform your authentication checks here.
        // For example, examine the URI and decide whether to proceed.
        // ...

        self.inner.call(req)
    }
}
