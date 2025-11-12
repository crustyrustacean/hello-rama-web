// src/main.rs

// dependencies
use hello_rama_web::config::Config;
use hello_rama_web::startup::Application;

#[tokio::main]
async fn main() {
    let config = Config::default();
    Application::build(config)
        .run()
        .await
        .expect("Unable to start the server.");
}
