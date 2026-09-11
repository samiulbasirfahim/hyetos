use super::detector::{Intent, detect};
use crate::db::DBPool;
use crate::types::message::Message;
use reqwest::Client;

pub async fn handle(msg: Message, pool: DBPool, client: &Client) -> Message {
    let intent = detect(&msg, client).await;

    let reply: String = match intent {
        Intent::Echo { text } => text,
        Intent::Connect => super::connect::connect(msg.get_platform(), &pool).await,
        Intent::Start => super::start::start(&pool, client, msg.get_platform()).await,

        Intent::Chat { text } => crate::services::ask_gemini(client, &text)
            .await
            .unwrap_or_else(|_| "I'm having trouble thinking right now.".to_string()),
    };

    Message::new(msg.get_platform().clone(), reply)
}
