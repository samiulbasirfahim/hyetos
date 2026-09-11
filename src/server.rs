use crate::db::DBPool;
use crate::routes;
use actix_web::{App, HttpServer, web};

pub async fn start(db_pool: DBPool) {
    let config = crate::Config::get();
    let client = reqwest::Client::new();

    println!("Starting server on port {}", config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(client.clone()))
            .configure(routes::register)
    })
    .bind(format!("0.0.0.0:{}", config.port))
    .unwrap_or_else(|e| panic!("{} - {}", config.port, e.to_string()))
    .run()
    .await
    .unwrap_or_else(|_| panic!("Failed to run server"));
}
