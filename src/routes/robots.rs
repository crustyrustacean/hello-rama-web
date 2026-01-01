// src/routes/robots.rs

// dependencies
use crate::errors::ApiError;
use rama::http::header::CONTENT_TYPE;
use rama::http::service::web::response::IntoResponse;

pub async fn robots_txt() -> Result<impl IntoResponse, ApiError> {
    Ok((
        [(CONTENT_TYPE, "text/plain")],
        include_str!("../../static/robots.txt"),
    ))
}