// src/state.rs

// dependencies
use tera::Tera;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: &'static Tera,
}

impl AppState {
    pub fn new(templates: &'static Tera) -> Self {
        Self { templates }
    }
}
