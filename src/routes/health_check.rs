// src/routes/health_check.rs

// dependencies
use rama::http::{StatusCode, service::web::response::IntoResponse};

// health_check endpoint which returns a 200 OK response and empty body
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
