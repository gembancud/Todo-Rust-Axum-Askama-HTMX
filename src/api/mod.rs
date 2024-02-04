use axum::Router;
use tower_http::trace::TraceLayer;

mod handlers;
mod middleware;
pub mod utils;

pub fn create_api() -> Router {
    handlers::create_router().layer(TraceLayer::new_for_http())
}
