// src/main.rs

// dependencies
use hello_rama_web::config::Config;
use hello_rama_web::startup::Application;
use rama::error::OpaqueError;

#[tokio::main]
async fn main() -> Result<(), OpaqueError> {
    let config = Config::default();
    Application::build(config).run().await?;

    Ok(())
}
