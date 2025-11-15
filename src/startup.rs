// src/startup.rs

// dependencies
use crate::config::Config;
use crate::routes::{get_index, health_check};
use rama::{
    error::BoxError,
    graceful::Shutdown, http::server::HttpServer, http::service::web::Router, rt::Executor,
    tcp::server::TcpListener,
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
    }

    pub async fn run(self) -> Result<(), BoxError> {
        let graceful = Shutdown::default();

        let tcp_listener = TcpListener::bind(self.config.address).await?;

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
