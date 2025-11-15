// src/routes/mod.rs

// module declarations
pub mod health_check;
pub mod get_html;
pub mod get_static_files;

// re-exports
pub use health_check::*;
pub use get_html::*;
pub use get_static_files::*;
