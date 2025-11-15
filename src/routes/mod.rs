// src/routes/mod.rs

// module declarations
pub mod get_html;
pub mod get_static_files;
pub mod health_check;

// re-exports
pub use get_html::*;
pub use get_static_files::*;
pub use health_check::*;
