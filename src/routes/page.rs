// src/routes/page.rs

// dependencies
use crate::errors::ApiError;
use crate::markdown::markdown_to_html;
use crate::state::AppState;
use rama::http::service::web::extract::{Path, State};
use rama::http::service::web::response::Html;
use std::fs;
use tera::Context;

#[derive(Debug, serde::Deserialize)]
pub struct Page {
    name: String,
}

#[derive(serde::Serialize)]
struct PageContext {
    title: String,
    msg: String,
}

pub async fn render_page(State(state): State<AppState>, Path(page): Path<Page>) -> Result<Html<String>, ApiError> {
          
    let page_title = page.name.clone();
    let content_name = format!("{}.md", page_title);
    let content_dir = format!("content/{}", content_name);
    let markdown_content = fs::read_to_string(content_dir).unwrap();
    let page_msg = markdown_to_html(&markdown_content);

    let page_context = PageContext {
        title: page_title,
        msg: page_msg,
    };

    let body = state
        .templates
        .render(&format!("{}.html", page.name), &Context::from_serialize(&page_context)?)
        .map_err(|e| ApiError::InternalServerError(format!("Template error: {}", e)))?;

    Ok(Html(body))
}
