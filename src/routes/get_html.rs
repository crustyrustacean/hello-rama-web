// src/routes/index.rs

// dependencies
use rama::http::StatusCode;
use rama::http::dep::http::Response;
use rama::http::service::web::response::IntoResponse;
use std::fs::File;
use std::io::Read;

// health_check endpoint which returns a 200 OK response and empty body
pub async fn get_index() -> impl IntoResponse {
    let mut file = File::open("templates/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(contents)
        .unwrap()
}
