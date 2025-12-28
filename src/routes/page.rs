// src/routes/page.rs

// dependencies
use crate::errors::ApiError;
use crate::state::AppState;
use rama::http::service::web::extract::State;
use rama::http::service::web::response::Html;
use tera::Context;

pub async fn get_index(State(state): State<AppState>) -> Result<Html<String>, ApiError> {
    let context = Context::new();

    let response_body = state.templates.render("base.html", &context)?;

    Ok(Html(response_body))
}
