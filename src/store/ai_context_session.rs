use std::sync::OnceLock;

use dashmap::DashMap;

use crate::types::context::{UserContext, UserMemory, UserMessage, UserSession};

type ContextStore = DashMap<String, UserSession>;
static CONTEXT_STORE: OnceLock<ContextStore> = OnceLock::new();

pub fn bootstrap() {
    CONTEXT_STORE.get_or_init(|| DashMap::new());
}

pub fn push_new_message(user_id: &str, user_text: String, ai_response: String) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| UserSession {
            messages: Vec::new(),
            last_interaction: chrono::Utc::now(),
            context: Default::default(),
            summary: None,
            memories: Vec::new(),
        });

    session.messages.push(UserMessage {
        user_text,
        ai_response,
        timestamp: chrono::Utc::now(),
    });

    session.last_interaction = chrono::Utc::now();
}

pub fn update_context(user_id: &str, user_context: UserContext) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| UserSession {
            messages: Vec::new(),
            last_interaction: chrono::Utc::now(),
            context: Default::default(),
            summary: None,
            memories: Vec::new(),
        });

    session.context = user_context;
}

pub fn update_summary(user_id: &str, summary: String) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| UserSession {
            messages: Vec::new(),
            last_interaction: chrono::Utc::now(),
            context: Default::default(),
            summary: None,
            memories: Vec::new(),
        });

    session.summary = Some(summary);
}

pub fn add_new_memories(user_id: &str, memory: UserMemory) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| UserSession {
            messages: Vec::new(),
            last_interaction: chrono::Utc::now(),
            context: Default::default(),
            summary: None,
            memories: Vec::new(),
        });

    session.memories.push(memory);
}

pub fn get_user_session(user_id: &str) -> Option<UserSession> {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);
    store.get(user_id).map(|entry| entry.clone())
}
