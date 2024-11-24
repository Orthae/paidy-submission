use std::sync::Arc;
use crate::infrastructure::connection_factory::{DatabaseConfiguration, PostgresConnectionPoolFactory};
use crate::web::item_endpoint::ItemRouter;
use axum::Router;

use crate::server::middleware::{RequestIdMiddleware, TraceMiddleware};
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::application::item_service::ItemServiceImpl;
use crate::domain::item_factory::ItemFactoryImpl;
use crate::infrastructure::item_repository::ItemRepositoryImpl;
use crate::server::configuration::Load;

pub struct ServerConfiguration {
    pub port: u16,
}

pub struct WebServer;

pub struct Context;

impl Context {
    pub async fn run() {
        tracing_subscriber::registry()
            .with(TraceMiddleware::filter())
            .with(tracing_subscriber::fmt::layer())
            .init();
        
        let pool = PostgresConnectionPoolFactory::new(DatabaseConfiguration::load()).await;
        let repository = Arc::new(ItemRepositoryImpl::new(pool));
        let factory = Arc::new(ItemFactoryImpl::default());
        let application = Arc::new(ItemServiceImpl::new(repository, factory));

        let router = ItemRouter::new(application.clone());
        let app = Router::new()
            .nest("/v1/", router)
            .layer(TraceMiddleware::new())
            .layer(RequestIdMiddleware::new());
        
        let listener = TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("Failed to bind to port 3000");
        axum::serve(listener, app)
            .await
            .expect("Failed to start server");
    }
}





