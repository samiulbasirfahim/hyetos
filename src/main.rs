use hyetos::server;

#[tokio::main]
async fn main() {
    server::start().await
}
