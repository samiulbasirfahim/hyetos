use std::time::Duration;

use crate::db::DBPool;
use crate::intent::handle::handle;
use crate::platform::telegram::Telegram;
use crate::types::platform::PlatformHandler;
use actix_web::{HttpResponse, Responder, web};

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

    let typing_client = client.clone();
    let typing_message = parsed_message.clone();
    let typing_task = actix_web::rt::spawn(async move {
        let _ = actix_web::rt::time::timeout(Duration::from_secs(90), async {
            let mut ticker = actix_web::rt::time::interval(std::time::Duration::from_secs(4));
            loop {
                ticker.tick().await;
                typing_message
                    .get_platform()
                    .send_typing_indicator(&typing_client)
                    .await;
            }
        })
        .await;
    });

    let handled_msg = handle(parsed_message, pool, client).await;
    typing_task.abort();
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

async fn discord_webhook(
    db: web::Data<DBPool>,
    client: web::Data<reqwest::Client>,
    body: web::Bytes,
) -> impl Responder {
    println!("Received Discord webhook: {:?}", body);
    HttpResponse::Ok().finish()
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/webhook")
            .route("/telegram", web::post().to(telegram_webhook))
            .route("/discord", web::post().to(discord_webhook)),
    );
}
