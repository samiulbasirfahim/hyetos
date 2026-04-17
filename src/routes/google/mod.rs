mod google_sign_in;
mod redirect_uri;
use actix_web::web::{self, ServiceConfig};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth").service(
            web::scope("/google")
                .route("/callback", web::get().to(redirect_uri::callback))
                .route("", web::get().to(google_sign_in::google_login)),
        ),
    );
}
