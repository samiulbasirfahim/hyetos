use crate::types::session::ExternalSession;
use dashmap::DashMap;
use std::sync::OnceLock;

type SessionStore = DashMap<String, ExternalSession>;

static SESSION_STORE: OnceLock<SessionStore> = OnceLock::new();

pub fn bootstrap() {
    SESSION_STORE.get_or_init(|| DashMap::new());
}

pub fn add(key: String, session: ExternalSession) -> Result<(), String> {
    let store = SESSION_STORE.get().ok_or("Session store not initialized")?;

    store.insert(key, session);
    Ok(())
}

pub fn get(key: &str) -> Option<impl std::ops::Deref<Target = ExternalSession>> {
    let store = match SESSION_STORE.get().ok_or("Session store not initialized") {
        Ok(store) => store,
        Err(_) => return None,
    };
    let session = store.get(key)?;
    Some(session)
}

pub fn delete(key: &str) -> Result<(), String> {
    let store = SESSION_STORE.get().ok_or("Session store not initialized")?;

    store.remove(key);
    Ok(())
}
