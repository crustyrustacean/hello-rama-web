// src/routes/health_check.rs

// dependencies
use rama::http::service::web::response::Html;

// health_check endpoint which returns a 200 OK response and empty body
pub async fn get_index() -> Html<String> {
    Html(r#"<h1>Hello, World!</h1>"#.to_string())
}
