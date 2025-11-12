// tests/api/health_check.rs

// dependencies
use crate::helpers::send_request;
use rama::http::StatusCode;

#[tokio::test]
async fn health_check_works() {
    let response = send_request("/health_check").await;
    assert_eq!(StatusCode::OK, response.status());
}
