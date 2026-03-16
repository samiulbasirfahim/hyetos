
use crate::types::message::IncomingMessage;
use crate::types::platform::{Platform, PlatformHandler};
use crate::types::telegram::TelegramUpdate;

pub struct Telegram;

impl PlatformHandler for Telegram {
    fn parse(&self, body: &[u8]) -> Option<IncomingMessage> {
        let update: TelegramUpdate = serde_json::from_slice(body).ok()?;
        let message = update.message?;
        let text = message.text?;
        let telegram_user_id = message.from?.id;
        let database_user_id = None;

        Some(IncomingMessage {
            platform: Platform::Telegram {
                user_id: telegram_user_id,
            },
            content: text,
            user_id: database_user_id,
        })
    }

    async fn send(&self, msg: crate::types::message::OutgoingMessage) {
        let config = crate::Config::get();

        let chat_id = match &msg.platform {
            Platform::Telegram { user_id } => user_id,
            _ => {
                println!("[TELEGRAM] wrong platform");
                return;
            }
        };

        match reqwest::Client::new()
            .post(format!("https://api.telegram.org/bot{}/sendMessage", {
                config.telegram_token.clone()
            }))
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": msg.content,
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
