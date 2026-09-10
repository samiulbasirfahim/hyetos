use super::platform::Platform;

#[derive(Debug)]
pub struct Message {
    platform: Platform,
    content: String,
}

impl Message {
    pub fn new(platform: Platform, content: String) -> Self {
        Self { platform, content }
    }
    pub async fn send(&self) {
        self.platform.send_message(self.get_content()).await;
    }
    pub fn get_content(&self) -> &str {
        &self.content.trim()
    }
    pub fn get_platform(&self) -> &Platform {
        &self.platform
    }
}
