use hyetos::server;

#[actix_web::main]
async fn main() {
    hyetos::Config::load();

    let db_poll = hyetos::db::connect_db().await;
    println!("Successfully connected to the database");

    server::start(db_poll).await
}
