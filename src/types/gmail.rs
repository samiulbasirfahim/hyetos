use chrono::{DateTime, Utc};

pub const MAX_PREVIOUS_SESSION_COUNT: usize = 5;

pub struct GmailSession {
    pub current: GmailContext,
    pub previous: Vec<GmailContext>,
}

pub struct GmailContext {
    pub search: GmailSearch,
    pub results: Vec<GmailMessageContext>,
    pub selected_message: Option<String>,
}

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

pub struct GmailSearch {
    pub id: String,
    pub snippet: Option<String>,
    pub only_unread: bool,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub has_attachments: Option<bool>,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>,
}
