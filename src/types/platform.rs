use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    Telegram { user_id: u64 },
}

#[allow(async_fn_in_trait)]
pub trait PlatformHandler {
    fn parse(&self, body: &[u8]) -> Option<Message>;
}

impl PartialEq for Platform {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Platform::Telegram { user_id: id1 }, Platform::Telegram { user_id: id2 }) => {
                id1 == id2
            }
        }
    }
}

impl Platform {
    pub fn parse_message(&self, body: &[u8]) -> Option<Message> {
        match self {
            Platform::Telegram { .. } => crate::platform::telegram::Telegram.parse(body),
        }
    }
    pub async fn send_message(&self, message: &str) {
        match self {
            Platform::Telegram { user_id } => {
                crate::platform::telegram::Telegram
                    .send(user_id, message)
                    .await;
            }
        }
    }
}
