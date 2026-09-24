use super::platform::Platform;
use std::sync::Arc;

#[derive(Debug)]
pub struct Message {
    platform: Arc<Platform>,
    content: String,
}

impl Message {
    pub fn new(platform: Platform, content: String) -> Self {
        Self {
            platform: Arc::new(platform),
            content,
        }
    }
    pub async fn send(&self, client: &reqwest::Client) {
        self.platform.send(&client, self.get_content()).await;
    }
    pub fn into_reply(self, reply_content: String) -> Self {
        Self {
            platform: self.platform,
            content: reply_content,
        }
    }
    pub fn get_content(&self) -> &str {
        &self.content.trim()
    }
    pub fn get_platform(&self) -> Arc<Platform> {
        Arc::clone(&self.platform)
    }
}
