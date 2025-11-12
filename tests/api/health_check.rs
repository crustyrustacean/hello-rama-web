// tests/api/health_check.rs

use hello_rama_web::Application;
use rama::Context;
use rama::Service;
use rama::http::{Body, Request, StatusCode};

#[tokio::test]
async fn test_health_check() {
    let service = Application::app_web_service();

    let request = Request::builder()
        .uri("/health_check")
        .body(Body::empty())
        .expect("Unable to send the request.");

    let ctx = Context::default();
    let response = service.serve(ctx, request).await.unwrap();

    assert_eq!(StatusCode::OK, response.status());
}
