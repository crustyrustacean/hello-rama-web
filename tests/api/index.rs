// tests/api/index.rs

// dependencies
use crate::helpers::send_request;
use rama::http::{BodyExtractExt, StatusCode};

#[tokio::test]
async fn render_index_page_returns_html() {
    // Act
    let response = send_request("/").await;
    let response_status = response.status();
    let response_header = response.headers().get("Content-Type").unwrap().to_owned();
    let response_body = response.try_into_string().await.unwrap();

    // Assert
    assert_eq!(StatusCode::OK, response_status);
    assert_eq!(response_header, "text/html; charset=utf-8");
    assert!(response_body.contains("<!DOCTYPE html>"))
}
