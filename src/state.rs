// src/state.rs

#[derive(Debug, Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
