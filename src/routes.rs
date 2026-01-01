// src/routes/mod.rs

// module declarations
pub mod health_check;
pub mod robots;
pub mod sitemap;
pub mod page;
pub mod update;

// re-exports
pub use health_check::*;
pub use robots::*;
pub use sitemap::*;
pub use page::*;
pub use update::*;
