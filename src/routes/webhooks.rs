use actix_web::{HttpResponse, Responder, web};
use crate::db::DBPool;
use crate::intent::handle::handle;
use crate::platform::telegram::Telegram;
use crate::types::platform::PlatformHandler;

async fn handle_webhook(
    platform: impl PlatformHandler,
    client: &reqwest::Client,
    pool: DBPool,
    body: &[u8],
) {
    let Some(parsed_message) = platform.parse(body) else {
        println!("Couldn't parse");
        return;
    };

    let handled_msg = handle(parsed_message, pool, client).await;
    handled_msg.send(client).await;
}

async fn telegram_webhook(
    db: web::Data<DBPool>,
    client: web::Data<reqwest::Client>,
    body: web::Bytes,
) -> impl Responder {
    println!("Received Telegram webhook: {:?}", body);
    handle_webhook(Telegram, client.get_ref(), db.get_ref().clone(), &body).await;
    HttpResponse::Ok().finish()
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/webhook").route("/telegram", web::post().to(telegram_webhook)));
}
