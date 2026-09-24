use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum ExternalSessionAction {
    Connect,
}

#[derive(Debug)]
pub struct ExternalSession {
    pub action: ExternalSessionAction,
    pub platform: String,
    pub exipres_at: DateTime<Utc>,
}
