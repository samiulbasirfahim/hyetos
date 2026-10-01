use crate::types::gmail::{GmailContext, GmailSession};
use dashmap::DashMap;
use std::sync::OnceLock;

type GmailContextPerUser = DashMap<String, GmailSession>;
const MAX_PREVIOUS_SESSION_COUNT: usize = 5;

static GMAIL_CONTEXT: OnceLock<GmailContextPerUser> = OnceLock::new();

fn create_context(user_id: &str, message_context: GmailContext, store: &GmailContextPerUser) {
    let session = GmailSession {
        current: message_context,
        previous: Vec::new(),
    };
    store.insert(user_id.to_string(), session);
}

pub fn bootstrap() {
    GMAIL_CONTEXT.get_or_init(|| DashMap::new());
}

pub fn add(user_id: &str, session: GmailContext) -> Result<(), String> {
    let store = GMAIL_CONTEXT
        .get()
        .ok_or("Gmail context store not initialized")?;

    match store.get_mut(user_id) {
        Some(mut existing_session) => {
            let current = std::mem::replace(&mut existing_session.current, session);
            existing_session.previous.push(current);
            if existing_session.previous.len() > MAX_PREVIOUS_SESSION_COUNT {
                existing_session.previous.remove(0);
            }
        }
        None => create_context(user_id, session, &store),
    }

    Ok(())
}

pub fn refine_current(user_id: &str, session: GmailContext) -> Result<(), String> {
    let store = GMAIL_CONTEXT
        .get()
        .ok_or("Gmail context store not initialized")?;

    match store.get_mut(user_id) {
        Some(mut existing_session) => {
            let _ = std::mem::replace(&mut existing_session.current, session);
        }
        None => create_context(user_id, session, &store),
    }

    Ok(())
}
