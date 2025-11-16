// src/routes/index.rs

// dependencies
use crate::assets::Templates;
use rama::http::StatusCode;
use rama::http::dep::http::Response;
use rama::http::service::web::response::IntoResponse;

// health_check endpoint which returns a 200 OK response and empty body
pub async fn get_index() -> impl IntoResponse {
    let index_html = Templates::get("index.html").unwrap();
    let contents = std::str::from_utf8(index_html.data.as_ref())
        .unwrap()
        .to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(contents)
        .unwrap()
}
