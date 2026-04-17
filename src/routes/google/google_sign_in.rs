use crate::services::build_oauth_client;
use crate::store::session;
use actix_web::cookie::Cookie;
use actix_web::{HttpResponse, Responder, web};
use oauth2::{CsrfToken, Scope};

#[derive(serde::Deserialize, Debug)]
pub struct GoogleLoginQuery {
    state: Option<String>,
}

pub async fn google_login(query: web::Query<GoogleLoginQuery>) -> impl Responder {
    let client = build_oauth_client();

    let state_from_query = match &query.state {
        Some(s) => s.clone(),
        None => {
            println!("[GOOGLE] No state provided in query");
            return HttpResponse::BadRequest().body("Missing state");
        }
    };

    match session::get(&state_from_query) {
        Some(state) => {
            if state.exipres_at < chrono::Utc::now() {
                println!("[GOOGLE] State expired: {}", state_from_query);
                return HttpResponse::BadRequest().body("State expired");
            }
        }
        None => {
            println!("[GOOGLE] Invalid state: {}", state_from_query);
            return HttpResponse::BadRequest().body("Invalid state");
        }
    }
    let (auth_url, csrf_token) = client
        .authorize_url(|| CsrfToken::new(state_from_query))
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

    let cookie = Cookie::build("oauth_state", csrf_token.secret().clone())
        .path("/")
        .http_only(true)
        .finish();

    return HttpResponse::Found()
        .insert_header(("Location", auth_url.to_string()))
        .cookie(cookie)
        .finish();
}
