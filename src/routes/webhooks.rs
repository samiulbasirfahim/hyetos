use actix_web::{HttpResponse, Responder, web};

use crate::db::DBPool;
use crate::intent::handle::handle;
use crate::platform::telegram::Telegram;
use crate::server::AppState;
use crate::types::platform::PlatformHandler;

async fn handle_webhook(platform: impl PlatformHandler, pool: DBPool, body: &[u8]) {
    let Some(incoming) = platform.parse(body) else {
        println!("Couldn't parse");

        return;
    };

    let outgoing = handle(incoming, pool).await;
    platform.send(outgoing).await;
}

async fn telegram_webhook(data: web::Data<AppState>, body: web::Bytes) -> impl Responder {
    handle_webhook(Telegram, data.db.clone(), &body).await;
    HttpResponse::Ok().finish()
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/webhook").route("/telegram", web::post().to(telegram_webhook)));
}
