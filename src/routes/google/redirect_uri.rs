use actix_web::{HttpRequest, HttpResponse, Responder, web};
use oauth2::TokenResponse;
use oauth2::{AuthorizationCode, reqwest::async_http_client};

use crate::services::build_oauth_client;
use crate::store::session;

#[derive(serde::Deserialize, Debug)]
pub struct CallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

pub async fn callback(req: HttpRequest, query: web::Query<CallbackQuery>) -> impl Responder {
    let client = build_oauth_client();

    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await;

    let query_state = match &query.state {
        Some(s) => s,
        None => return HttpResponse::BadRequest().body("Missing state"),
    };

    let cookie_state = req.cookie("oauth_state").map(|c| c.value().to_string());

    if cookie_state.is_none() || cookie_state.as_ref().unwrap() != query_state {
        return HttpResponse::BadRequest().body("Invalid CSRF state");
    }

    let session = match session::get(query_state) {
        Some(s) => s,
        None => return HttpResponse::BadRequest().body("Invalid session state"),
    };

    if session.exipres_at < chrono::Utc::now() {
        return HttpResponse::BadRequest().body("Session expired");
    }

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

            println!(
                "[GOOGLE] Auth successful for user: {} ({}), for session: {:#?}",
                name, email, &*session
            );

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
