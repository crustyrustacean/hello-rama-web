// tests/api/health_check.rs

// dependencies
use crate::helpers::send_request;
use rama::http::StatusCode;

#[tokio::test]
async fn health_check_works() {
    // Act
    let response = send_request("/api/v1/health_check").await;

    // Assert
    assert_eq!(StatusCode::OK, response.status());
}
