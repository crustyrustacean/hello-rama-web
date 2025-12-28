// src/lib.rs

// module declarations
pub mod configuration;
pub mod errors;
pub mod response;
pub mod routes;
pub mod startup;
pub mod state;
pub mod telemetry;

// re-exports
pub use configuration::*;
pub use errors::*;
pub use response::*;
pub use startup::*;
pub use state::*;
pub use telemetry::*;
