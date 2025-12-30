// src/routes/update.rs

// dependencies
use rama::http::service::web::response::Html;

pub async fn update_message() -> Html<String> {
    Html("<div id=\"test\">I've been updated!</div><div id=\"reset\"><button data-on:click=@get('/api/v1/reset')>Reset Me!</button></div>".to_string())
}

pub async fn reset_message() -> Html<String> {
    Html("<div id=\"test\"></div><div id=\"reset\"></div>".to_string())
}
