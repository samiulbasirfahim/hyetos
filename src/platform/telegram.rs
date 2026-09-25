use crate::types::message::Message;
use crate::types::platform::{Platform, PlatformHandler};
use crate::types::telegram::TelegramUpdate;
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct Telegram {
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub message_id: i64,
    pub username: Option<String>,
}

impl PlatformHandler for Telegram {
    fn parse(body: &[u8]) -> Option<Message> {
        let update: TelegramUpdate = serde_json::from_slice(body).ok()?;

        let message = update.message?;
        let text = message.text?;

        let message_from = message.from?;

        let state = Telegram {
            user_id: message_from.id,
            group_id: Some(message.chat.id),
            message_id: message.message_id,
            username: message_from.username,
        };

        Some(Message::new(Platform::Telegram(state), text))
    }

    fn user_string_to_platform(user_str: &str) -> Option<Platform> {
        let id_str = user_str.trim_start_matches("telegram_user:");
        if let Ok(id) = id_str.parse::<i64>() {
            return Some(Platform::Telegram(Telegram {
                user_id: id,
                group_id: None,
                message_id: 0,
                username: None,
            }));
        }
        None
    }

    async fn send(&self, client: &Client, msg: &str) {
        let config = crate::Config::get();
        let target_chat = self.group_id.unwrap_or(self.user_id);

        let payload = serde_json::json!({
            "chat_id": target_chat,
            "text": msg,
            "reply_parameters": { "message_id": self.message_id }
        });

        match client
            .post(format!(
                "https://api.telegram.org/bot{}/sendMessage",
                config.telegram_token
            ))
            .json(&payload)
            .send()
            .await
        {
            Ok(_) => {}
            Err(x) => println!("[TELEGRAM] Failed to send message, {}", x),
        };
    }

    async fn send_typing_indicator(&self, client: &Client) {
        let config = crate::Config::get();
        let target_chat = self.group_id.unwrap_or(self.user_id);

        println!(
            "[TELEGRAM] Sending typing indicator to user_id: {}",
            self.user_id
        );

        let url = format!(
            "https://api.telegram.org/bot{}/sendChatAction",
            config.telegram_token
        );
        let response_result = client
            .post(&url)
            .json(&serde_json::json!({
                "chat_id": target_chat,
                "action": "typing",
            }))
            .send()
            .await;

        match response_result {
            Ok(res) if !res.status().is_success() => {
                let error_body = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                println!(
                    "[TELEGRAM] API rejected typing indicator. Response: {}",
                    error_body
                );
            }
            Err(e) => println!("[TELEGRAM] Network request failed: {}", e),
            _ => {}
        }
    }

    fn get_user(&self) -> String {
        format!("telegram_user:{}", self.user_id)
    }

    fn is_group_chat(&self) -> bool {
        self.group_id.is_some() && self.group_id != Some(self.user_id)
    }

    fn get_session_key(&self) -> String {
        let target_id = self.group_id.unwrap_or(self.user_id);
        format!("telegram:{}", target_id)
    }
}
