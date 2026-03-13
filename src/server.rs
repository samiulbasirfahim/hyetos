use crate::db::DBPool;
use crate::routes;
use actix_web::{App, HttpServer, web};

pub struct AppState {
    pub db: DBPool,
}

pub async fn start(db_pool: DBPool) {
    let config = crate::Config::get();

    let app_state = web::Data::new(AppState {
        db: db_pool.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::register)
    })
    .bind(format!("0.0.0.0:{}", config.port))
    .unwrap_or_else(|_| panic!("PORT {} is in use", config.port))
    .run()
    .await
    .unwrap_or_else(|_| panic!("Failed to run server"));
}
