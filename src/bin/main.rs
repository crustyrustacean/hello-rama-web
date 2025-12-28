// src/main.rs

// dependencies
use hello_rama_web::configuration::get_configuration;
use hello_rama_web::errors::{AppBoxError, AppErrorContext, AppOpaqueError};
use hello_rama_web::startup::Application;
use hello_rama_web::telemetry::{get_subscriber, init_subscriber};
use rama::telemetry::tracing;

#[tokio::main]
async fn main() -> Result<(), AppBoxError> {
    // initialize tracing
    let subscriber = get_subscriber(
        "hello-rama-web".into(),
        "info,rama=debug".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // build the app configuration
    tracing::info!("Reading app configuration...");
    let configuration = get_configuration().expect("Failed to read configuration");

    // build and run the application
    tracing::info!("Building the application...");
    Application::build(configuration)
        .await
        .map_err(AppOpaqueError::from_boxed)
        .context("Unable to build the server on the configured host and port.")?
        .run()
        .await
        .map_err(AppOpaqueError::from_boxed)
        .context("Unable to run the server")?;

    Ok(())
}
