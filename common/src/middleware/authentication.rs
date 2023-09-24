use tonic::{Request, Status};
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
