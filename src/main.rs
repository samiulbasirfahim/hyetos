use hyetos::server;

#[actix_web::main]
async fn main() {
    hyetos::Config::load();
    hyetos::store::session::bootstrap();

    actix_web::rt::spawn(hyetos::workers::cleanup::session_cleanup());

    let db_pool = hyetos::db::connect_db().await;
    println!("Successfully connected to the database");

    server::start(db_pool).await
}
