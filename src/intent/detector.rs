use crate::types::message::IncomingMessage;
use crate::utils;

#[derive(Debug)]
pub enum Intent {
    Echo { text: String },
    Unknown,
}

pub async fn detect(msg: &IncomingMessage) -> Intent {
    let text = msg.content.trim();

    let is_command = text.starts_with('/');
    if !is_command {
        return Intent::Unknown;
    }

    let command = utils::text::parse_command(text);

    match command.0 {
        "/echo" => Intent::Echo {
            text: String::from(command.1),
        },
        _ => Intent::Unknown,
    }
}
