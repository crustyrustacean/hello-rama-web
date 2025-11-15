// tests/api/get_index.rs

// dependencies
use crate::helpers::send_request;
use rama::http::{
    StatusCode,
    headers::{ContentType, HeaderMapExt, dep::mime},
};

#[tokio::test]
async fn get_index_returns_200_ok_and_html() {
    // Act
    let response = send_request("/").await;

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        response.headers().typed_get::<ContentType>().unwrap(),
        ContentType::from(mime::TEXT_HTML_UTF_8)
    );
}
