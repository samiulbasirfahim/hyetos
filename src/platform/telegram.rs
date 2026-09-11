use crate::types::message::Message;
use crate::types::platform::{Platform, PlatformHandler};
use crate::types::telegram::TelegramUpdate;
use reqwest::Client;

pub struct Telegram;

impl PlatformHandler for Telegram {
    fn parse(&self, body: &[u8]) -> Option<Message> {
        let update: TelegramUpdate = serde_json::from_slice(body).ok()?;
        println!("[TELEGRAM] Received update: {:?}", update);
        let message = update.message?;
        let text = message.text?;
        let telegram_user_id = message.from?.id;
        Some(Message::new(
            Platform::Telegram {
                user_id: telegram_user_id,
            },
            text,
        ))
    }

    async fn send(&self, client: &Client, chat_id: &i64, msg: &str) {
        let config = crate::Config::get();

        match client
            .post(format!("https://api.telegram.org/bot{}/sendMessage", {
                config.telegram_token.clone()
            }))
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": msg,
            }))
            .send()
            .await
        {
            Ok(_) => {}
            Err(x) => {
                println!("[TELEGRAM] Failed to send message, {}", x);
            }
        };
    }
    async fn send_typing_indicator(&self, client: &reqwest::Client, user_id: &i64) {
        let config = crate::Config::get();
        println!(
            "[TELEGRAM] Sending typing indicator to user_id: {}",
            user_id
        );

        let url = format!(
            "https://api.telegram.org/bot{}/sendChatAction",
            config.telegram_token
        );

        let response_result = client
            .post(&url)
            .json(&serde_json::json!({
                "chat_id": user_id,
                "action": "typing",
            }))
            .send()
            .await;

        match response_result {
            Ok(res) => {
                if !res.status().is_success() {
                    let error_body = res
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    println!(
                        "[TELEGRAM] API rejected typing indicator. Response: {}",
                        error_body
                    );
                }
            }
            Err(e) => {
                println!("[TELEGRAM] Network request failed: {}", e);
            }
        }
    }
}
