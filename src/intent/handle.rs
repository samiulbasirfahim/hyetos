use super::detector::{Intent, detect};
use crate::db::DBPool;
use crate::types::message::Message;
use reqwest::Client;

pub async fn handle(msg: Message, pool: DBPool, client: &Client) -> Option<Message> {
    let intent = detect(&msg, client).await;

    let reply: String = match intent {
        Intent::Echo { text } => text,
        Intent::Connect => super::connect::connect(msg.get_platform(), &pool).await,
        Intent::Start => super::start::start(&pool, msg.get_platform()).await,
        Intent::Ignore => {
            println!("Ignoring message: {:?}", msg);
            return None;
        }
        Intent::Chat { text } => text,
        Intent::GmailSearch { search } => {
            format!("Searching Gmail with criteria: {:?}", search)
        }
    };
    let reply = msg.into_reply(reply);
    Some(reply)
}
