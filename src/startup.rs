// src/startup.rs

// dependencies
use crate::configuration::Settings;
use crate::errors::AppBoxError;
use crate::errors::AppErrorContext;
use crate::errors::AppOpaqueError;
use crate::routes::health_check;
use crate::state::AppState;
use crate::telemetry::make_request_span;
use rama::{
    Layer, error::BoxError, graceful::Shutdown, http::layer::trace::TraceLayer,
    http::server::HttpServer, http::service::web::Router, rt::Executor, tcp::server::TcpListener,
    telemetry::tracing,
};
use std::time::Duration;

pub struct Application {
    pub router: Router<AppState>,
    pub listener: TcpListener,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, AppBoxError> {
        let state = AppState::default();
        let router = Self::build_app_router(state);
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)
            .await
            .map_err(AppOpaqueError::from_boxed)
            .context(format!(
                "Unable to create TCP listener on: Host: {}, Port: {}",
                configuration.application.host, configuration.application.port
            ))?;

        tracing::info!(
            "Listening on: Host: {}, Port: {}",
            configuration.application.host,
            configuration.application.port
        );
        Ok(Self { router, listener })
    }

    pub fn build_app_router(state: AppState) -> Router<AppState> {
        tracing::info!("Health check enabled at: /health_check");
        Router::new_with_state(state).with_get("/health_check", health_check)
    }

    pub async fn run(self) -> Result<(), BoxError> {
        let graceful = Shutdown::default();

        let router = self.router;
        let listener = self.listener;

        tracing::info!("Running the application...");
        graceful.spawn_task_fn(async |guard| {
            let exec = Executor::graceful(guard.clone());
            let http_service = HttpServer::auto(exec).service(
                TraceLayer::new_for_http()
                    .make_span_with(make_request_span)
                    .into_layer(router),
            );
            listener.serve_graceful(guard, http_service).await;
        });

        graceful
            .shutdown_with_limit(Duration::from_secs(10))
            .await?;

        Ok(())
    }
}
