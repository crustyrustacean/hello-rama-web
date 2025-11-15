// src/main.rs

// dependencies
use hello_rama_web::config::Config;
use hello_rama_web::startup::Application;
use rama::error::{BoxError, ErrorContext, OpaqueError};

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let config = Config::default();
    Application::build(config)
        .run()
        .await
        .map_err(OpaqueError::from_boxed)
        .context("Unable to start the server")?;

    Ok(())
}
