mod health;
use actix_web::{HttpResponse, web};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.configure(health::register);
    cfg.route("/", web::to(HttpResponse::Ok));
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn test_root_return_200() {
        let app = test::init_service(App::new().configure(register)).await;

        let req = test::TestRequest::get().uri("/").to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), 200, "/ doesn't return status code 200");
    }
}
