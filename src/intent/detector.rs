use crate::ai::detect_intent::detect_intent;
use crate::types::gmail::GmailSearch;
use crate::types::message::Message;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum Intent {
    Echo {
        text: String,
    },
    Connect,
    GmailSearch {
        search_query: GmailSearch,
        shallow: bool,
        id_in_prev: bool,
    },
    Chat {
        text: String,
    },
    Ignore,
}

pub async fn detect(msg: &Message, client: &Client) -> Intent {
    let text = msg.get_content().trim();
    if text.starts_with('/') {
        let (command_raw, payload) = text.split_once(' ').unwrap_or((text, ""));
        let command = command_raw
            .split_once('@')
            .map(|(c, _)| c)
            .unwrap_or(command_raw);

        return match command.to_lowercase().as_str() {
            "/echo" => Intent::Echo {
                text: payload.to_string(),
            },
            "/connect" => Intent::Connect,
            _ => Intent::Chat {
                text: format!("Unknown command: {command}. Send /start for a list of commands."),
            },
        };
    }

    match detect_intent(client, text, &msg.get_platform().get_user()).await {
        Ok(intent) => intent,
        Err(err) => {
            eprintln!("Error detecting intent: {err}");
            Intent::Echo {
                text: "Sorry, I couldn't understand your message. Please try again.".to_string(),
            }
        }
    }
}
