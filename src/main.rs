use paidy_submission::server::server::WebServer;

#[tokio::main]
async fn main() {
    WebServer::run().await;
}