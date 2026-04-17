use crate::db::DBPool;
use crate::types::message::{IncomingMessage, OutgoingMessage};
use crate::types::platform::Platform;

use super::detector::{Intent, detect};

pub async fn handle(msg: IncomingMessage, platform: Platform, pool: DBPool) -> OutgoingMessage {
    let intent = detect(&msg).await;

    let reply: String = match intent {
        Intent::Echo { text } => text.to_string(),
        Intent::Connect => super::connect::connect(&platform).await,
        Intent::Unknown => "Unknown command. Try /help".to_string(),
    };

    OutgoingMessage {
        platform: msg.platform,
        content: reply,
    }
}
