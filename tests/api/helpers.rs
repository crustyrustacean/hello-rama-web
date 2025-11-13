// test/api/helpers

// dependencies
use hello_rama_web::Application;
use rama::Context;
use rama::Service;
use rama::http::{Body, Request, Response};

pub async fn send_request(uri: &str) -> Response {
    let service = Application::app_web_service();
    let request = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .expect("Failed to build request");

    service
        .serve(Context::default(), request)
        .await
        .expect("Failed to execute request")
}
