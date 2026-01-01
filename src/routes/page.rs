// src/routes/page.rs

// dependencies
use crate::errors::ApiError;
use crate::markdown::markdown_to_html;
use crate::state::AppState;
use chrono::{Datelike, Local};
use rama::http::StatusCode;
use rama::http::service::web::extract::State;
use rama::http::service::web::response::Html;
use tera::Context;

#[derive(serde::Serialize)]
struct PageContext {
    page_title: String,
    page_content: Option<String>,
    footer_year: i32,
}

pub async fn home_page(State(state): State<AppState>) -> Result<Html<String>, ApiError> {
    let index_markdown = include_str!("../../content/pages/index.md");
    let page_content = markdown_to_html(index_markdown);

    let context = PageContext {
        page_title: "Home".to_string(),
        page_content: Some(page_content),
        footer_year: Local::now().year(),
    };

    let body = state
        .templates
        .render("index.html", &Context::from_serialize(&context)?)
        .map_err(|e| ApiError::InternalServerError(format!("Template error: {}", e)))?;

    Ok(Html(body))
}

pub async fn not_found(
    State(state): State<AppState>,
) -> Result<(StatusCode, Html<String>), ApiError> {
    let context = PageContext {
        page_title: "Not Found".to_string(),
        page_content: Some("Nothing here by that name".to_string()),
        footer_year: Local::now().year(),
    };

    let body = state
        .templates
        .render("404.html", &Context::from_serialize(&context)?)
        .map_err(|e| ApiError::InternalServerError(format!("Template error: {}", e)))?;

    Ok((StatusCode::NOT_FOUND, Html(body)))
}
