// src/startup.rs

// dependencies
use crate::config::Config;
use crate::errors::AppBoxError;
use crate::errors::AppErrorContext;
use crate::errors::AppOpaqueError;
use crate::routes::health_check;
use rama::{
    error::BoxError, graceful::Shutdown, http::server::HttpServer, http::service::web::Router,
    rt::Executor, tcp::server::TcpListener, telemetry::tracing,
};
use std::time::Duration;

pub struct Application {
    pub router: Router<()>,
    pub listener: TcpListener,
}

impl Application {
    pub async fn build(config: Config) -> Result<Self, AppBoxError> {
        let router = Self::build_app_router();
        let address = format!("{}:{}", config.host, config.port);
        let listener = TcpListener::bind(address)
            .await
            .map_err(AppOpaqueError::from_boxed)
            .context(format!(
                "Unable to create TCP listener on: Host: {}, Port: {}",
                config.host, config.port
            ))?;

        tracing::info!("Listening on: Host: {}, Port: {}", config.host, config.port);
        Ok(Self { router, listener })
    }

    pub fn build_app_router() -> Router<()> {
        tracing::info!("Health check enabled at: /health_check");
        Router::new().with_get("/health_check", health_check)
    }

    pub async fn run(self) -> Result<(), BoxError> {
        let graceful = Shutdown::default();

        let router = self.router;
        let listener = self.listener;

        tracing::info!("Running the application...");
        graceful.spawn_task_fn(async move |guard| {
            let exec = Executor::graceful(guard.clone());
            let http_service = HttpServer::auto(exec).service(router);
            listener.serve_graceful(guard, http_service).await;
        });

        graceful
            .shutdown_with_limit(Duration::from_secs(10))
            .await?;

        Ok(())
    }
}
