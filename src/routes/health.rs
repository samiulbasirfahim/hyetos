use actix_web::web;

pub async fn health() -> &'static str {
    return "HELLO";
}

pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(health)));
}
