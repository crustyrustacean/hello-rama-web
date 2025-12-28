// test/api/helpers

// dependencies
use hello_rama_web::startup::Application;
use hello_rama_web::state::AppState;
use hello_rama_web::telemetry::{get_subscriber, init_subscriber, make_request_span};
use rama::Layer;
use rama::Service;
use rama::http::layer::trace::TraceLayer;
use rama::http::{Body, Request, Response};
use std::sync::LazyLock;

// Ensure that the `tracing` stack is only initialised once
pub static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub async fn send_request(uri: &str) -> Response {
    LazyLock::force(&TRACING);

    let state = AppState::default();
    let router = Application::build_app_router(state);
    let service = TraceLayer::new_for_http()
        .make_span_with(make_request_span)
        .into_layer(router);

    let request = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .expect("Failed to build request");

    let response = service
        .serve(request)
        .await
        .expect("Failed to execute request");

    response.map(Body::new)
}
