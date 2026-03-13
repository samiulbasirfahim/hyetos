use super::platform::Platform;
use chrono::{DateTime, Utc};

pub enum ExternalSessionAction {
    Connect,
    Register,
}

pub struct ExternalSession {
    action: ExternalSessionAction,
    uri: String,
    platform: Platform,
    exipres_at: DateTime<Utc>,
}
