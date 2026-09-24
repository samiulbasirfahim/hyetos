use crate::platform::{Discord, Telegram};
use crate::types::message::Message;
use reqwest::Client;

#[allow(async_fn_in_trait)]
pub trait PlatformHandler {
    fn parse(body: &[u8]) -> Option<Message>;
    fn user_string_to_platform(user_str: &str) -> Option<Platform>;

    async fn send(&self, client: &Client, msg: &str);
    async fn send_typing_indicator(&self, client: &Client);
    fn get_session_key(&self) -> String;
    fn get_user(&self) -> String;
    fn is_group_chat(&self) -> bool;
}

#[derive(Debug)]
pub enum Platform {
    Telegram(Telegram),
    Discord(Discord),
}

macro_rules! delegate {
    ($self:ident . $method:ident ( $( $arg:expr ),* ) ) => {
        match $self {
            Platform::Telegram(data) => data.$method( $( $arg ),* ),
            Platform::Discord(data) => data.$method( $( $arg ),* ),
        }
    };

    ($self:ident . $method:ident ( $( $arg:expr ),* ) . await ) => {
        match $self {
            Platform::Telegram(data) => data.$method( $( $arg ),* ).await,
            Platform::Discord(data) => data.$method( $( $arg ),* ).await,
        }
    };
}

impl Platform {
    pub fn parse_webhook(platform_name: &str, body: &[u8]) -> Option<Message> {
        match platform_name {
            "telegram" => Telegram::parse(body),
            "discord" => Discord::parse(body),
            _ => None,
        }
    }

    pub fn string_to_platform(user_str: &str) -> Option<Platform> {
        let (platform_name, _) = user_str.split_once(':')?;
        match platform_name {
            "telegram" => Telegram::user_string_to_platform(user_str),
            "discord" => Discord::user_string_to_platform(user_str),
            _ => None,
        }
    }

    pub async fn send(&self, client: &Client, msg: &str) {
        delegate!(self.send(client, msg).await)
    }

    pub async fn send_typing_indicator(&self, client: &Client) {
        delegate!(self.send_typing_indicator(client).await)
    }

    pub fn get_session_key(&self) -> String {
        delegate!(self.get_session_key())
    }

    pub fn get_user(&self) -> String {
        delegate!(self.get_user())
    }

    pub fn is_group_chat(&self) -> bool {
        delegate!(self.is_group_chat())
    }
}
