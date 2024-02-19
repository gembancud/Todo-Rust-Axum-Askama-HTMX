mod todo;
use crate::api::middleware::htmx_trigger_middleware;
use axum::{middleware, Router};

use tower_http::services::ServeDir;

pub fn create_router() -> Router {
    let router = Router::new()
        .nest("/todo", todo::create_router())
        .nest_service("/static", ServeDir::new("src/api/templates"))
        .layer(middleware::from_fn(htmx_trigger_middleware));
    router
}
