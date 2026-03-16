use super::message::{IncomingMessage, OutgoingMessage};

#[derive(Debug)]
pub enum Platform {
    Telegram { user_id: u64 },
    Discord { user_id: String },
}

#[allow(async_fn_in_trait)]
pub trait PlatformHandler {
    fn parse(&self, body: &[u8]) -> Option<IncomingMessage>;
    async fn send(&self, msg: OutgoingMessage);
}
