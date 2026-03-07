use crate::db::DBPool;
use crate::routes;
use axum::{Router, routing::get};
use tokio;

pub async fn start(db_pool: DBPool) {
    let config = crate::Config::get();
    let app: Router = build_server().with_state(db_pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap_or_else(|err| match err.kind() {
            std::io::ErrorKind::AddrInUse => {
                panic!("PORT {} is using by other process", config.port)
            }
            _ => panic!("Failed to bind at port {}", config.port),
        });

    println!("Succesfully bind the server at PORT: {}", config.port);

    axum::serve(listener, app).await.unwrap();
}

fn build_server() -> Router<DBPool> {
    Router::new()
        .route("/", get(routes::check_running))
        .route("/health", get(routes::health))
}
