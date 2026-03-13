use crate::types::telegram::TelegramUpdate;
use actix_web::{HttpResponse, Responder, web};

async fn telegram_webhook(body: web::Json<TelegramUpdate>) -> impl Responder {
    println!("Called the webhook, {:#?}", body);
    HttpResponse::Ok().finish()
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/webhook").route("/telegram", web::post().to(telegram_webhook)));
}
