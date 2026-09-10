use crate::db::DBPool;
use crate::types::message::Message;

use super::detector::{Intent, detect};

pub async fn handle(msg: Message, pool: DBPool) -> Message {
    let intent = detect(&msg).await;

    let reply: String = match intent {
        Intent::Echo { text } => text.to_string(),
        Intent::Connect => super::connect::connect(msg.get_platform(), &pool).await,
        Intent::Start => super::start::start(&pool, msg.get_platform()).await,
        Intent::Unknown => "Unknown command. Try /help".to_string(),
    };

    Message::new(msg.get_platform().clone(), reply)
}
