use serde::{Deserialize, Serialize};

use super::gmail::GmailQueryResponse;

// to lazy to implement setter function, so leaving everything public for now. Will fix later if needed.
#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectIntentReturn {
    AuthorizedEmail(AuthorizedInfo),
    ConnectionUrl(ConnectionUrl),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionUrl {
    pub url: String,
    pub connecting_to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedInfo {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Start {
    pub message: String,
    pub connected_info: Option<AuthorizedInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IntentReturn {
    Connect(ConnectIntentReturn),
    Chat(String),
    Start(Start),
    IntentHandlerError(String),
    GmailResult(GmailQueryResponse),
}
