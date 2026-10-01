use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct GmailSession {
    pub current: GmailContext,
    pub previous: Vec<GmailContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmailContext {
    pub search: GmailSearch,
    pub results: Vec<GmailMessageContext>,
    pub selected_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmailMessageContext {
    pub id: String,
    pub snippet: String,
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    pub date: DateTime<Utc>,
    pub unread: bool,
    pub has_attachments: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmailSearch {
    pub text: Option<String>,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,

    pub read_state: Option<ReadState>,
    pub starred: Option<bool>,
    pub important: Option<bool>,

    pub has_attachments: Option<bool>,

    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>,

    pub larger_than: Option<u64>,
    pub smaller_than: Option<u64>,

    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadState {
    Read,
    Unread,
}
