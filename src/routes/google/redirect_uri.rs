use actix_web::{HttpRequest, HttpResponse, Responder, web};
use oauth2::TokenResponse;
use oauth2::{AuthorizationCode, reqwest::async_http_client};
use sqlx::PgPool;

use crate::models::Auth;
use crate::services::build_oauth_client;
use crate::store::session;

#[derive(serde::Deserialize, Debug)]
pub struct CallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

pub async fn callback(
    req: HttpRequest,
    query: web::Query<CallbackQuery>,
    db: web::Data<PgPool>,
    client_reqwest: web::Data<reqwest::Client>,
) -> impl Responder {
    let client = build_oauth_client();

    let query_state = match &query.state {
        Some(s) => s,
        None => return HttpResponse::BadRequest().body("Missing state parameter"),
    };

    let cookie_state = req.cookie("oauth_state").map(|c| c.value().to_string());

    if cookie_state.is_none() || cookie_state.as_ref().unwrap() != query_state {
        return HttpResponse::BadRequest().body("Invalid CSRF state. Security check failed.");
    }

    let session = match session::get(query_state) {
        Some(s) => s,
        None => return HttpResponse::BadRequest().body("Invalid or missing session state"),
    };

    if session.exipres_at < chrono::Utc::now() {
        return HttpResponse::BadRequest().body("Session expired. Please try logging in again.");
    }

    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret().clone();
            let refresh_token = match token.refresh_token() {
                Some(rt) => rt.secret().clone(),
                None => {
                    println!("[GOOGLE] WARNING: No refresh token returned by Google!");
                    return HttpResponse::InternalServerError().body("No refresh token returned. Revoke access in Google settings and try again.");
                }
            };

            let granted_scopes: Vec<String> = match token.scopes() {
                Some(scopes) => scopes.iter().map(|s| s.as_ref().to_string()).collect(),
                None => {
                    println!("[GOOGLE] Scopes omitted in response");
                    vec!["openid".to_string()]
                }
            };

            let user_info = match reqwest::Client::new()
                .get("https://www.googleapis.com/oauth2/v2/userinfo")
                .bearer_auth(&access_token)
                .send()
                .await
            {
                Ok(res) => res.json::<serde_json::Value>().await.unwrap_or_default(),
                Err(e) => {
                    println!("[GOOGLE] Failed to fetch user profile: {}", e);
                    return HttpResponse::InternalServerError()
                        .body("Failed to fetch Google profile");
                }
            };

            let email = user_info["email"].as_str().unwrap_or("");
            let name = user_info["name"].as_str().unwrap_or("");

            let platform_users_check =
                Auth::get_platform_users_by_mail_address(db.get_ref(), email).await;

            match platform_users_check {
                Err(sqlx::Error::RowNotFound) => {
                    let insert_result = Auth::insert(
                        db.get_ref(),
                        vec![session.platform.clone()],
                        refresh_token.clone(),
                        email.to_string(),
                        granted_scopes.clone(),
                    )
                    .await;
                    println!("[DB] Inserted new user: {}", email);

                    if let Err(e) = insert_result {
                        println!("[DB ERROR] Failed to insert new user: {}", e);
                        return HttpResponse::InternalServerError()
                            .body("Failed to save new user to database");
                    }
                }

                Ok(mut users) => {
                    if !users.contains(&session.platform) {
                        users.push(session.platform.clone());
                    }
                    println!("[DB] Updating existing user: {}", email);

                    let update_result = Auth::update_existing(
                        db.get_ref(),
                        users,
                        refresh_token.clone(),
                        email.to_string(),
                        granted_scopes.clone(),
                    )
                    .await;

                    if let Err(e) = update_result {
                        println!("[DB ERROR] Failed to update existing user: {}", e);
                        return HttpResponse::InternalServerError()
                            .body("Failed to update user in database");
                    }
                }

                Err(e) => {
                    println!("[DB CRITICAL] Database query failed: {}", e);
                    return HttpResponse::InternalServerError().body("Database connection error");
                }
            }

            session
                .platform
                .send_message(
                    client_reqwest.get_ref(),
                    &format!(
                        "Successfully linked your Google account: {} ({})",
                        name, email
                    ),
                )
                .await;

            HttpResponse::Ok().json(serde_json::json!({
                "status":            "ok",
                "email":             email,
                "name":              name,
            }))
        }

        Err(e) => {
            println!("[GOOGLE] Token exchange failed: {}", e);
            HttpResponse::InternalServerError().body("Google authentication failed")
        }
    }
}
