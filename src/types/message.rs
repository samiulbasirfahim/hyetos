use super::platform::Platform;

#[derive(Debug)]
pub struct IncomingMessage {
    pub platform: Platform,
    pub content: String,
    pub user_id: Option<i64>,
}

#[derive(Debug)]
pub struct OutgoingMessage {
    pub platform: Platform,
    pub content: String,
}
