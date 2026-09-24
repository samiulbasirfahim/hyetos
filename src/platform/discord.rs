use crate::types::message::Message;
use crate::types::platform::{Platform, PlatformHandler};
use reqwest::Client;

#[derive(Debug)]
pub struct Discord {
    pub user_id: String,
    pub channel_id: Option<String>,
}

impl PlatformHandler for Discord {
    fn parse(_body: &[u8]) -> Option<Message> {
        None
    }

    fn user_string_to_platform(user_str: &str) -> Option<Platform> {
        let id_str = user_str.trim_start_matches("discord:");
        Some(Platform::Discord(Discord {
            user_id: id_str.to_string(),
            channel_id: None,
        }))
    }

    async fn send(&self, _client: &Client, msg: &str) {
        println!(
            "[DISCORD DUMMY] Sending message to {}: {}",
            self.user_id, msg
        );
    }

    async fn send_typing_indicator(&self, _client: &Client) {
        println!("[DISCORD DUMMY] Typing indicator for {}", self.user_id);
    }

    fn get_user(&self) -> String {
        format!("discord_user_{}", self.user_id)
    }

    fn is_group_chat(&self) -> bool {
        self.channel_id.is_some()
    }

    fn get_session_key(&self) -> String {
        let target = self.channel_id.as_deref().unwrap_or(&self.user_id);
        format!("discord:{}", target)
    }
}
