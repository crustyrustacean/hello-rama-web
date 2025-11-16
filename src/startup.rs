// src/startup.rs

// dependencies
use crate::config::Config;
use crate::routes::{get_css_file, get_image_file, get_index, get_scripts_file, health_check};
use rama::error::{ErrorContext, OpaqueError};
use rama::{
    error::BoxError, graceful::Shutdown, http::server::HttpServer, http::service::web::Router,
    rt::Executor, tcp::server::TcpListener,
};
use std::time::Duration;

pub struct Application {
    pub router: Router<()>,
    pub config: Config,
}

impl Application {
    pub fn build(config: Config) -> Self {
        let router = Self::app_web_service();

        Self { router, config }
    }

    pub fn app_web_service() -> Router<()> {
        Router::new()
            .get("/health_check", health_check)
            .get("/", get_index)
            .get("/static/styles.css", get_css_file)
            .get("/static/scripts.js", get_scripts_file)
            .get("/static/favicon.png", get_image_file)
    }

    pub async fn run(self) -> Result<(), BoxError> {
        let graceful = Shutdown::default();

        let tcp_listener = TcpListener::bind(self.config.address)
            .await
            .map_err(OpaqueError::from_boxed)
            .context("Unable to create TCP listener")?;

        graceful.spawn_task_fn(async |guard| {
            let exec = Executor::graceful(guard.clone());
            let http_service = HttpServer::auto(exec).service(self.router);
            tcp_listener.serve_graceful(guard, http_service).await;
        });

        graceful
            .shutdown_with_limit(Duration::from_secs(2))
            .await?;

        Ok(())
    }
}
