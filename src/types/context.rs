use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub messages: Vec<UserMessage>,
    pub last_interaction: DateTime<Utc>,
    pub context: UserContext,
    pub summary: Option<String>,
    pub memories: Vec<UserMemory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    pub user_text: String,
    pub ai_response: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserContext {
    pub preferred_response_format: Option<String>,
    pub preferred_response_language: Option<String>,
    pub preferred_response_tone: Option<String>,
    pub preferred_response_length: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMemory {
    pub content: String,
    pub category: MemoryCategory,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryCategory {
    Preference,
    PersonalFact,
    ProfessionalContext,
    Project,
    Instruction,
}
