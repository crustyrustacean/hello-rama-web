// tests/api/get_index.rs

// dependencies
use crate::helpers::send_request;
use rama::http::{BodyExtractExt, StatusCode};

#[tokio::test]
async fn get_index_returns_200_ok_and_html() {
    // Act
    let response = send_request("/").await;
    let (parts, body) = response.into_parts();

    // Assert
    assert_eq!(StatusCode::OK, parts.status);
    assert_eq!(
        parts.headers.get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );

    let response_body = body.try_into_string().await.expect("Unable to get a string from the message body");
    assert!(response_body.contains("<h1>Hello, World!"));
}
