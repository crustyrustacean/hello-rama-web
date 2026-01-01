// src/startup.rs

// dependencies
use crate::configuration::Settings;
use crate::errors::AppBoxError;
use crate::errors::AppErrorContext;
use crate::errors::AppOpaqueError;
use crate::routes::{
    health_check, not_found, home_page, reset_message, update_message, robots_txt, sitemap_xml
};
use crate::state::AppState;
use crate::telemetry::make_request_span;
use crate::templates::compile_templates;
use rama::{
    Layer,
    error::BoxError,
    graceful::Shutdown,
    http::layer::trace::TraceLayer,
    http::server::HttpServer,
    http::service::fs::DirectoryServeMode::NotFound,
    http::service::web::{Router, response::DatastarScript},
    rt::Executor,
    tcp::server::TcpListener,
    telemetry::tracing,
};
use std::time::Duration;

pub struct Application {
    pub router: Router<AppState>,
    pub listener: TcpListener,
}

impl Application {
    pub async fn build(configuration: &Settings) -> Result<Self, AppBoxError> {
        // compile the app templates
        let compiled_templates = compile_templates(configuration)?;

        // build app state
        let state = AppState::new(compiled_templates);

        // build the app router
        let router = Self::build_app_router(state);

        // configure the host and port
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
        Router::new_with_state(state)
            .with_sub_router_make_fn("/api", |router| {
                router.with_sub_router_make_fn("/v1", |router| {
                    router
                        .with_get("/health_check", health_check)
                        .with_get("/update", update_message)
                        .with_get("/reset", reset_message)
                })
            })
            .with_get("/", home_page)
            .with_get("/static/datastar.js", DatastarScript::default())
            .with_get("/robots.txt", robots_txt)
            .with_get("/sitemap.xml", sitemap_xml)
            .with_dir_and_serve_mode("/static", "static", NotFound)
            .with_not_found(not_found)
    }

    pub async fn run(self, configuration: &Settings) -> Result<(), BoxError> {
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
            .shutdown_with_limit(Duration::from_secs(
                configuration.application.shutdown_timeout,
            ))
            .await?;

        Ok(())
    }
}
