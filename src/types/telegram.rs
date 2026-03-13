use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TelegramUpdate {
    pub update_id: u64,
    pub message: Option<TelegramMessage>,
}

#[derive(Deserialize, Debug)]
pub struct TelegramMessage {
    pub message_id: u64,
    pub from: Option<TelegramUser>,
    pub chat: TelegramChat,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TelegramChat {
    pub id: i64,
}

#[derive(Deserialize, Debug)]
pub struct TelegramUser {
    pub id: u64,
    pub first_name: String,
    pub username: Option<String>,
}
