// tests/api/get_static_assets.rs

// dependencies
use crate::helpers::send_request;
use rama::http::{
    StatusCode,
    headers::{ContentType, HeaderMapExt, dep::mime},
};

#[tokio::test]
async fn get_static_assets_returns_200_ok_and_css_content() {
    // Act
    let response = send_request("/static/styles.css").await;

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        response.headers().typed_get::<ContentType>().unwrap(),
        ContentType::from(mime::TEXT_CSS_UTF_8)
    );
}

#[tokio::test]
async fn get_static_assets_returns_200_ok_and_javascript_content() {
    // Act
    let response = send_request("/static/scripts.js").await;

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        response.headers().typed_get::<ContentType>().unwrap(),
        ContentType::from(mime::TEXT_JAVASCRIPT)
    );
}