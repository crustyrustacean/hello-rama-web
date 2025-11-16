// src/routes/static_files.rs

// dependencies
use crate::assets::Asset;
use rama::http::Body;
use rama::http::Response;
use rama::http::StatusCode;
use rama::http::service::web::response::IntoResponse;

// endpoint which returns a 200 OK response and CSS in the body
pub async fn get_css_file() -> impl IntoResponse {
    let css_file = Asset::get("styles.css").unwrap();
    let contents = std::str::from_utf8(css_file.data.as_ref())
        .unwrap()
        .to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/css; charset=utf-8")
        .body(contents)
        .unwrap()
}

// endpoint which returns a 200 OK response and CSS in the body
pub async fn get_scripts_file() -> impl IntoResponse {
    let scripts_file = Asset::get("scripts.js").unwrap();
    let contents = std::str::from_utf8(scripts_file.data.as_ref())
        .unwrap()
        .to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/javascript")
        .body(contents)
        .unwrap()
}

// endpoint which returns a 200 OK response and CSS in the body
pub async fn get_image_file() -> impl IntoResponse {
    let image_file = Asset::get("favicon.png").unwrap();
    let contents = image_file.data.as_ref().to_vec();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "image/png")
        .body(Body::from(contents))
        .unwrap()
}
