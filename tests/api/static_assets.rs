// tests/api/static_assets.rs

// dependencies
use crate::helpers::send_request;
use rama::http::StatusCode;

#[tokio::test]
async fn static_route_handler_serves_css() {
    // Arrange
    let css_asset = "screen.css";
    
    // Act
    let response = send_request(&format!("/static/{}", css_asset)).await;
    let response_header = response.headers().get("Content-Type").unwrap();

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(response_header, "text/css");
}

#[tokio::test]
async fn static_route_handler_serves_javascript() {
    // Arrange
    let js_asset = "scripts.js";

    // Act
    let response = send_request(&format!("/static/{}", js_asset)).await;
    let response_header = response.headers().get("Content-Type").unwrap();

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(response_header, "text/javascript");
}

#[tokio::test]
async fn static_route_handler_serves_image_icon() {
    // Arrange
    let image_asset = "favicon.png";

    // Act
    let response = send_request(&format!("/static/{}", image_asset)).await;
    let response_header = response.headers().get("Content-Type").unwrap();

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(response_header, "image/png");
}