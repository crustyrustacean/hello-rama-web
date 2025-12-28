// src/main.rs

// dependencies
use hello_rama_web::config::Config;
use hello_rama_web::errors::{AppBoxError, AppErrorContext, AppOpaqueError};
use hello_rama_web::startup::Application;
use rama::telemetry::tracing::{
    self,
    level_filters::LevelFilter,
    subscriber::{self, EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt},
};

#[tokio::main]
async fn main() -> Result<(), AppBoxError> {
    // initialize tracing
    tracing::info!("Initialize tracing...");
    subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    // build the app configuration
    tracing::info!("Reading app configuration...");
    let config = Config::default();

    // build and run the application
    tracing::info!("Building the application...");
    Application::build(config)
        .await
        .map_err(AppOpaqueError::from_boxed)
        .context("Unable to build the server on the configured host and port.")?
        .run()
        .await
        .map_err(AppOpaqueError::from_boxed)
        .context("Unable to run the server")?;

    Ok(())
}
