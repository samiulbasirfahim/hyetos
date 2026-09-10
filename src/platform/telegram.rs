use crate::types::message::Message;
use crate::types::platform::{Platform, PlatformHandler};
use crate::types::telegram::TelegramUpdate;

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
}

impl Telegram {
    pub async fn send(&self, chat_id: &u64, msg: &str) {
        let config = crate::Config::get();

        match reqwest::Client::new()
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
}
