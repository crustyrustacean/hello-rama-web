// src/lib.rs

// module declarations
pub mod config;
pub mod errors;
pub mod routes;
pub mod startup;

// re-exports
pub use config::*;
pub use errors::*;
pub use startup::*;
