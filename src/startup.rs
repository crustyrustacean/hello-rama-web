// src/startup.rs

// dependencies
use crate::config::Config;
use crate::routes::{get_css_styles, get_images, get_index, get_javascript, health_check};
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
            .get("/static/styles.css", get_css_styles)
            .get("/static/scripts.js", get_javascript)
            .get("/static/favicon.png", get_images)
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
            .shutdown_with_limit(Duration::from_secs(30))
            .await?;

        Ok(())
    }
}
