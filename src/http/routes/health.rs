use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

/// Endpoint for CaaS services.
pub fn router() -> Router {
    Router::new().route("/health", get(handler))
}

async fn handler() -> impl IntoResponse {
    "Service is Alive"
}
