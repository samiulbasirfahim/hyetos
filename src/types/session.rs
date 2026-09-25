use chrono::{DateTime, Utc};

// using external session action type gotta implemenet partial google auth scope.
// Just connect only connect to google account, [FOR chat session so multiple platform can chat with
// sharing data]
// ConnectCalendar for connecting google calendar.
// ConnectMail for connecting google mail.
// ConnectAll for connecting all above service.

#[derive(Debug, Clone)]
pub enum ExternalSessionAction {
    Connect,
}

#[derive(Debug, Clone)]
pub struct ExternalSession {
    pub action: ExternalSessionAction,
    pub platform: String,
    pub exipres_at: DateTime<Utc>,
}
