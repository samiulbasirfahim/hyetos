use super::detector::{Intent, detect};
use crate::db::DBPool;
use crate::types::message::Message;
use reqwest::Client;

pub async fn handle(msg: Message, pool: DBPool, client: &Client) -> Option<Message> {
    let intent = detect(&msg, client).await;

    let reply: String = match intent {
        Intent::Echo { text } => text,
        Intent::Connect => super::connect::connect(msg.get_platform().as_ref(), &pool).await,
        Intent::Start => super::start::start(&pool, client, msg.get_platform().as_ref()).await,
        Intent::CreateEvent { date, title } => {
                format!("Creating event '{}' on {}...", title, date)
        }
        Intent::RetrieveEvents { date, date2 } => {
                format!(
                    "Retrieving events from {} to {}...",
                    date,
                    date2.unwrap_or_else(|| date.clone())
                )
        }
        Intent::Ignore => {
            println!("Ignoring message: {:?}", msg);
            return None;
        }

        Intent::Chat { text } => crate::services::ask_gemini(client, format!("
You're a helpful assistant, you mainly manages Events, using google calendar.
Please respond to the following message in a concise and friendly manner: {}
CRITICAL INSTRUCTION: Return strictly plain text. Do NOT use markdown, bolding, asterisks, or any special formatting.
                ", text).as_str())
            .await
            .unwrap_or_else(|_| "I'm having trouble thinking right now.".to_string()),
    };
    let reply = msg.into_reply(reply);
    Some(reply)
}
