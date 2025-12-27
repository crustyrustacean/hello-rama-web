// src/main.rs

// dependencies
use hello_rama_web::config::Config;
use hello_rama_web::errors::{AppBoxError, AppErrorContext, AppOpaqueError};
use hello_rama_web::startup::Application;

#[tokio::main]
async fn main() -> Result<(), AppBoxError> {
    let config = Config::default();
    Application::build(config)
        .await
        .map_err(AppOpaqueError::from_boxed)
        .context("Unable to build the server")?
        .run()
        .await
        .map_err(AppOpaqueError::from_boxed)
        .context("Unable to run the server")?;

    Ok(())
}
