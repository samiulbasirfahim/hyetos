use crate::services::ask_gemini;
use crate::types::message::Message;
use reqwest::Client;

#[derive(Debug)]
pub enum Intent {
    Echo { text: String },
    Start,
    Connect,
    Chat { text: String },
}

pub async fn detect(msg: &Message, client: &Client) -> Intent {
    let text = msg.get_content().trim();

    if text.starts_with('/') {
        let parts: Vec<&str> = text.splitn(2, ' ').collect();
        let command = parts[0];
        let payload = if parts.len() > 1 { parts[1] } else { "" };

        return match command.to_lowercase().as_str() {
            "/echo" => Intent::Echo {
                text: payload.to_string(),
            },
            "/connect" => Intent::Connect,
            "/start" => Intent::Start,
            _ => Intent::Chat {
                text: text.to_string(),
            },
        };
    }

    let prompt = format!(
        "You are a strict intent classifier for a bot. \
        Classify the following user message into exactly ONE of these categories:\n\
        - CONNECT (if the user wants to log in, link an account, or authenticate)\n\
        - START (if the user is saying hello or asking for an intro/onboarding)\n\
        - CHAT (for anything else, general questions, or conversation)\n\n\
        User Message: \"{}\"\n\n\
        CRITICAL: Respond with EXACTLY one word from the list above. No punctuation.",
        text
    );

    let classification = ask_gemini(client, &prompt).await.unwrap_or_default();

    match classification.trim().to_uppercase().as_str() {
        "CONNECT" => Intent::Connect,
        "START" => Intent::Start,
        _ => Intent::Chat {
            text: text.to_string(),
        },
    }
}
