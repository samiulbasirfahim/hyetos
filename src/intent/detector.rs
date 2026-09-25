use crate::services::ask_gemini;
use crate::types::message::Message;
use reqwest::Client;

#[derive(Debug)]
pub enum Intent {
    Echo { text: String },
    Start,
    Connect,
    CreateEvent { date: String, title: String },
    RetrieveEvents { date: String, date2: Option<String> },
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
            "/create-event" => {
                let (date, title) = payload.split_once(' ').unwrap_or(("", ""));
                if date.is_empty() || title.is_empty() {
                    return Intent::Chat {
                        text: "Usage: /create-event <date> <title>".to_string(),
                    };
                }
                Intent::CreateEvent {
                    date: date.to_string(),
                    title: title.to_string(),
                }
            }
            "/start" => Intent::Start,
            _ => Intent::Chat {
                text: format!("Unknown command: {command}. Send /start for a list of commands."),
            },
        };
    }

    let system_instruction = crate::prompt::build_system_instruction();

    let response = ask_gemini(client, &system_instruction, text)
        .await
        .unwrap_or_else(|_| "I'm having trouble thinking right now.".to_string());

    match response.trim() {
        "CONNECT" => Intent::Connect,
        "START" => Intent::Start,
        reply => Intent::Chat {
            text: reply.to_string(),
        },
    }
}
