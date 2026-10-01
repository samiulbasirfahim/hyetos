use crate::ai::detect_intent::detect_intent;
use crate::types::gmail::GmailSearch;
use crate::types::message::Message;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum Intent {
    Echo { text: String },
    Start,
    Connect,

    GmailSearch { search: GmailSearch },

    Chat { text: String },

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
            "/start" => Intent::Start,
            _ => Intent::Chat {
                text: format!("Unknown command: {command}. Send /start for a list of commands."),
            },
        };
    }

    detect_intent(client, text).await
}
