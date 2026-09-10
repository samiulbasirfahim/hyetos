use super::platform::Platform;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub enum ExternalSessionAction {
    Connect,
}

#[derive(Clone, Debug)]
pub struct ExternalSession {
    pub action: ExternalSessionAction,
    pub platform: Platform,
    pub exipres_at: DateTime<Utc>,
}
