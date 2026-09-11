use reqwest::Client;

use crate::db::DBPool;
use crate::models::Auth;
use crate::services::ask_gemini;
use crate::types::platform::Platform;

pub async fn start(pool: &DBPool, client: &Client, platform: &Platform) -> String {
    let user_result = Auth::get_user_by_platform_user(pool, platform).await;

    let prompt = match user_result {
        Ok(user) => format!(
            "Generate a concise, friendly welcome back message for a user returning to the Hyetos Bot. \
            You may subtly reference their email: {}. \
            CRITICAL INSTRUCTION: Return strictly plain text. Do NOT use markdown, bolding, asterisks, or any special formatting.",
            user.mail_address
        ),

        Err(sqlx::Error::RowNotFound) => String::from(
            "Generate a concise, warm welcome message for a brand new user who has just connected to the Hyetos Bot for the first time. \
            CRITICAL INSTRUCTION: Return strictly plain text. Do NOT use markdown, bolding, asterisks, or any special formatting.",
        ),

        Err(e) => {
            println!("[DB ERROR] Failed to fetch user for welcome intent: {}", e);
            String::from(
                "Generate a generic, friendly greeting message for the Hyetos Bot. \
                CRITICAL INSTRUCTION: Return strictly plain text. Do NOT use markdown, bolding, asterisks, or any special formatting.",
            )
        }
    };

    let ask_gemini = ask_gemini(client, &prompt).await;

    match ask_gemini {
        Ok(text) => text,
        Err(e) => {
            println!("[GEMINI ERROR] Failed to generate welcome message: {}", e);
            String::from("Welcome to Hyetos! We are glad you are here.")
        }
    }
}
