use paidy_submission::server::web_server::WebServer;

#[tokio::main]
async fn main() {
    WebServer::run().await;
}
