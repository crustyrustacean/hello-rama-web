// tests/api/not_found.rs

// dependencies
use crate::helpers::send_request;
use rama::http::{BodyExtractExt, StatusCode};

#[tokio::test]
async fn render_not_found_returns_404_for_nonexistant_routes() {
    // Act
    let response = send_request("/shouldnotexist").await;
    let response_status = response.status();
    let response_header = response.headers().get("Content-Type").unwrap().to_owned();
    let response_body = response.try_into_string().await.unwrap();

    // Assert
    assert_eq!(StatusCode::NOT_FOUND, response_status);
    assert_eq!(response_header, "text/html; charset=utf-8");
    assert!(response_body.contains("<!DOCTYPE html>"))
}
