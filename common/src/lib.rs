use tokio::task::JoinHandle;

mod db;
mod service;
pub use db::{MongoCollection, MongoDB};
pub use service::Service;
pub mod middleware;

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
