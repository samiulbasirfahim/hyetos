use std::sync::OnceLock;

use dashmap::DashMap;

use crate::db::DBPool;
use crate::models::UserSessionModel;
use crate::types::context::{ContextStore, UserContext, UserMemory, UserMessage, UserSession};

static CONTEXT_STORE: OnceLock<ContextStore> = OnceLock::new();

pub fn bootstrap() {
    CONTEXT_STORE.get_or_init(|| DashMap::new());
}

fn create_user_session_struct(db_pool: DBPool, user_id: &str) -> UserSession {
    let user = UserSession {
        messages: Vec::new(),
        last_interaction: chrono::Utc::now(),
        context: Default::default(),
        summary: None,
        memories: Vec::new(),
    };

    let user_clone = user.clone();
    let id_string = user_id.to_string();

    actix_web::rt::spawn(async move {
        save_user_session(db_pool, &id_string, &user_clone).await;
    });

    user
}

async fn save_user_session(db_pool: DBPool, user_id: &str, user_session: &UserSession) {
    let _ = UserSessionModel::save(&db_pool, user_id, user_session).await;
}

pub async fn push_new_message(
    user_id: &str,
    user_text: String,
    ai_response: String,
    db_pool: DBPool,
) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| create_user_session_struct(db_pool.clone(), user_id));

    session.messages.push(UserMessage {
        user_text,
        ai_response,
        timestamp: chrono::Utc::now(),
    });

    session.last_interaction = chrono::Utc::now();
}

pub fn update_context(user_id: &str, user_context: UserContext, db_pool: DBPool) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| create_user_session_struct(db_pool.clone(), user_id));

    session.context = user_context;

    actix_web::rt::spawn(async move {
        save_user_session(db_pool, session.key(), &session).await;
    });
}

pub fn update_summary(user_id: &str, summary: String, db_pool: DBPool) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| create_user_session_struct(db_pool.clone(), user_id));

    session.summary = Some(summary);
}

pub fn add_new_memories(user_id: &str, memory: UserMemory, db_pool: DBPool) {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);

    let mut session = store
        .entry(user_id.to_string())
        .or_insert_with(|| create_user_session_struct(db_pool.clone(), user_id));

    session.memories.push(memory);
}

pub fn get_user_session(user_id: &str, _db_pool: DBPool) -> Option<UserSession> {
    let store = CONTEXT_STORE.get_or_init(DashMap::new);
    let user_in_cache = store.get(user_id).map(|entry| entry.clone());

    if let Some(user) = user_in_cache {
        Some(user)
    } else {
        // fetch user_sessions from the db
        None
    }
}
