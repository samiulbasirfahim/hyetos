use crate::db::DBPool;
use crate::types::message::{IncomingMessage, OutgoingMessage};

use super::detector::{Intent, detect};

pub async fn handle(msg: IncomingMessage, pool: DBPool) -> OutgoingMessage {
    let intent = detect(&msg).await;

    let reply: String = match intent {
        Intent::Echo { text } => text.to_string(),
        Intent::Unknown => "Unknown command. Try /help".to_string(),
    };

    OutgoingMessage {
        platform: msg.platform,
        content: reply,
    }
}
