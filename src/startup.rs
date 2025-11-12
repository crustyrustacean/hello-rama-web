// src/startup.rs

// dependencies
use crate::config::Config;
use crate::routes::health_check;
use rama::{http::server::HttpServer, http::service::web::Router, rt::Executor};

pub struct Application {
    executor: Executor,
    pub router: Router<()>,
    pub config: Config,
}

impl Application {
    pub fn build(config: Config) -> Self {
        let executor = Executor::default();
        let router = Self::app_web_service();

        Self {
            executor,
            router,
            config,
        }
    }

    pub fn app_web_service() -> Router<()> {
        Router::new().get("/health_check", health_check)
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        HttpServer::auto(self.executor)
            .listen(&self.config.address, self.router)
            .await?;

        Ok(())
    }
}
