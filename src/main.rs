// src/main.rs

// dependencies
use rama::{
    http::service::web::Router,
    http::service::web::response::IntoResponse,
    http::{StatusCode, server::HttpServer},
    rt::Executor,
};

// health_check endpoint which returns a 200 OK response and empty body
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let exec = Executor::default();
    let router = Router::new().get("/health_check", health_check);
    HttpServer::auto(exec)
        .listen("127.0.0.1:8080", router)
        .await
        .expect("Unable to start the server at 127.0.0.1:8080");
}
