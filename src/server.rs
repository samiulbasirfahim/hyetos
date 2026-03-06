use crate::routes;
use axum::{Router, routing::get};
use tokio;

const PORT: u16 = 9999;

pub async fn start() {
    let app: Router = build_server();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap_or_else(|err| match err.kind() {
            std::io::ErrorKind::AddrInUse => panic!("PORT {} is using by other process", PORT),
            _ => panic!("Failed to bind at port {}", PORT),
        });

    println!("Succesfully bind the server at PORT: {}", PORT);

    axum::serve(listener, app).await.unwrap();
}

fn build_server() -> Router {
    Router::new()
        .route("/", get(routes::check_running))
        .route("/health", get(routes::health))
}
