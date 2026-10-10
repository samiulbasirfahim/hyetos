use super::detector::{Intent, detect};
use crate::ai::make_response::make_response;
use crate::db::DBPool;
use crate::types::intent_return::IntentReturn;
use crate::types::message::Message;
use reqwest::Client;

pub async fn handle(msg: Message, pool: DBPool, client: &Client) -> Option<Message> {
    let intent = detect(&msg, client).await;

    let reply: IntentReturn = match intent {
        Intent::Echo { text } => {
            return Some(msg.into_reply(text));
        }
        Intent::Connect => super::connect::connect(msg.get_platform(), &pool).await,
        Intent::Ignore => {
            println!("Ignoring message: {:?}", msg);
            return None;
        }
        Intent::Chat { text } => IntentReturn::Chat(text),
        Intent::GmailSearch {
            id_in_prev,
            shallow,
            search_query,
        } => {
            return Some(msg.into_reply(serde_json::to_string("GMAIL QUERY").unwrap()));
        }
    };
    let reply = make_response(client, reply, &msg.get_platform().get_user()).await;
    let reply = msg.into_reply(reply);
    Some(reply)
}
