// src/routes/page.rs

// dependencies
use crate::errors::ApiError;
use crate::markdown::markdown_to_html;
use crate::state::AppState;
use rama::http::StatusCode;
use rama::http::service::web::extract::State;
use rama::http::service::web::response::Html;
use tera::Context;
use chrono::{Datelike, Local};

#[derive(serde::Serialize)]
struct IndexPageContext {
    page_title: String,
    page_content: String,
    footer_year: i32,
}

pub async fn render_page_home(State(state): State<AppState>) -> Result<Html<String>, ApiError> {
    let index_page_title = "Home".to_string();
    let index_markdown = include_str!("../../content/index.md");
    let index_page_content = markdown_to_html(index_markdown);

    let index_page_context = IndexPageContext {
        page_title: index_page_title,
        page_content: index_page_content,
        footer_year: Local::now().year()
    };

    let body = state
        .templates
        .render("index.html", &Context::from_serialize(&index_page_context)?)
        .map_err(|e| ApiError::InternalServerError(format!("Template error: {}", e)))?;

    Ok(Html(body))
}

pub async fn render_not_found(State(state): State<AppState>) -> Result<(StatusCode, Html<String>), ApiError> {
    let mut context = Context::new();
    let not_found_page_title = "404 Not Found".to_string();
    let footer_year = Local::now().year();
    context.insert("page_title", &not_found_page_title);
    context.insert("footer_year", &footer_year);

    let body = state
        .templates
        .render("404.html", &context)
        .map_err(|e| ApiError::InternalServerError(format!("Template error: {}", e)))?;

    Ok((StatusCode::NOT_FOUND, Html(body)))
}
