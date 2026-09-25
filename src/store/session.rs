use crate::types::session::ExternalSession;
use chrono::Utc;
use dashmap::DashMap;
use std::sync::OnceLock;

type SessionStore = DashMap<String, ExternalSession>;

static SESSION_STORE: OnceLock<SessionStore> = OnceLock::new();

pub fn bootstrap() {
    SESSION_STORE.get_or_init(|| DashMap::new());
}

pub fn delete_expired() {
    let session = match SESSION_STORE.get() {
        Some(s) => s,
        None => return,
    };
    let current_now = Utc::now();
    session.retain(|_, y| y.exipres_at < current_now);
}

pub fn add(key: String, session: ExternalSession) -> Result<(), String> {
    let store = SESSION_STORE.get().ok_or("Session store not initialized")?;

    store.insert(key, session);
    Ok(())
}

pub fn get(key: &str) -> Option<ExternalSession> {
    let store = match SESSION_STORE.get().ok_or("Session store not initialized") {
        Ok(store) => store,
        Err(_) => return None,
    };
    println!("[SESSION] Retrieving session for key: {}", key);
    let session = store.get(key)?.clone();
    Some(session)
}

pub fn delete(key: &str) -> Result<(), String> {
    let store = SESSION_STORE.get().ok_or("Session store not initialized")?;

    store.remove(key);
    Ok(())
}
