use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web::{self, ServiceConfig};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl, basic::BasicClient, reqwest::async_http_client,
};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.route(
        "/auth/google/callback",
        web::get().to(google_callback_handler),
    )
    .route("/auth/google", web::get().to(google_login));
}

fn build_oauth_client() -> BasicClient {
    let config = crate::Config::get();

    BasicClient::new(
        ClientId::new(config.google_web_client_id.clone()),
        Some(ClientSecret::new(config.google_web_client_secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(config.google_redirect_uri.clone()).unwrap())
}

#[derive(serde::Deserialize, Debug)]
pub struct CallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

async fn google_callback_handler(query: web::Query<CallbackQuery>) -> impl Responder {
    let client = build_oauth_client();

    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();

            let user_info = reqwest::Client::new()
                .get("https://www.googleapis.com/oauth2/v2/userinfo")
                .bearer_auth(access_token)
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            println!("[GOOGLE] user: {:#?}", user_info);

            let email = user_info["email"].as_str().unwrap_or("");
            let name = user_info["name"].as_str().unwrap_or("");
            let google_id = user_info["id"].as_str().unwrap_or("");

            HttpResponse::Ok().json(serde_json::json!({
                "status":    "ok",
                "email":     email,
                "name":      name,
                "google_id": google_id,
            }))
        }
        Err(e) => {
            println!("[GOOGLE] token exchange failed: {}", e);
            HttpResponse::InternalServerError().body("Auth failed")
        }
    }
}

async fn google_login() -> impl Responder {
    let client = build_oauth_client();

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .url();

    return HttpResponse::Found()
        .insert_header(("Location", auth_url.to_string()))
        .finish();
}
