use crate::types::message::Message;
use crate::utils;

#[derive(Debug)]
pub enum Intent {
    Echo { text: String },
    Start,
    Connect,
    Unknown,
}

pub async fn detect(msg: &Message) -> Intent {
    let text = msg.get_content();

    let is_command = text.starts_with('/');
    if !is_command {
        return Intent::Unknown;
    }

    let command = utils::text::parse_command(text);

    match command.0 {
        "/echo" => Intent::Echo {
            text: String::from(command.1),
        },
        "/connect" => Intent::Connect,
        "/start" => Intent::Start,
        _ => Intent::Unknown,
    }
}
