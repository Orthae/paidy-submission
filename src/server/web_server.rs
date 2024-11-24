use crate::application::item_service::ItemServiceImpl;
use crate::domain::item_factory::ItemFactoryImpl;
use crate::infrastructure::connection_factory::{
    DatabaseConfiguration, PostgresConnectionPoolFactory,
};
use crate::infrastructure::item_repository::ItemRepositoryImpl;
use crate::server::configuration::Load;
use crate::server::middleware::{RequestIdMiddleware, TraceMiddleware};
use crate::web::item_endpoint::ItemRouter;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct WebServer;

impl WebServer {
    pub async fn run() {
        tracing_subscriber::registry()
            .with(TraceMiddleware::filter())
            .with(tracing_subscriber::fmt::layer())
            .init();

        info!("Application starting");
        info!("Loading configuration");
        let config = DatabaseConfiguration::load();

        info!("Creating database connection pool and running migrations");
        let pool = PostgresConnectionPoolFactory::create(config).await;

        info!("Creating repository");
        let repository = Arc::new(ItemRepositoryImpl::new(pool));

        info!("Creating item factory");
        let factory = Arc::new(ItemFactoryImpl);

        info!("Creating service");
        let application = Arc::new(ItemServiceImpl::new(repository, factory));

        info!("Creating router");
        let router = ItemRouter::create(application.clone());

        let app = Router::new()
            .nest("/v1/", router)
            .route("/health", get(|| async { StatusCode::OK }))
            .layer(TraceMiddleware::create())
            .layer(RequestIdMiddleware::create());

        info!("Binding application server");
        let listener = TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("Failed to bind to port 3000");

        info!("Starting application server");
        axum::serve(listener, app)
            .await
            .expect("Failed to start server");
    }
}
