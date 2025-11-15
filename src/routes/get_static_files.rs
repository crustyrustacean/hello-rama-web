// src/routes/static_files.rs

// dependencies
use rama::http::Body;
use rama::http::Response;
use rama::http::StatusCode;
use rama::http::service::web::response::IntoResponse;
use std::fs::File;
use std::io::Read;

// endpoint which returns a 200 OK response and CSS in the body
pub async fn get_css_styles() -> impl IntoResponse {
    let mut file = File::open("static/styles.css").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/css; charset=utf-8")
        .body(contents)
        .unwrap()
}

// endpoint which returns a 200 OK response and CSS in the body
pub async fn get_javascript() -> impl IntoResponse {
    let mut file = File::open("static/scripts.js").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/javascript")
        .body(contents)
        .unwrap()
}

// endpoint which returns a 200 OK response and CSS in the body
pub async fn get_images() -> impl IntoResponse {
    let mut file = File::open("static/favicon.png").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "image/png")
        .body(Body::from(contents))
        .unwrap()
}
