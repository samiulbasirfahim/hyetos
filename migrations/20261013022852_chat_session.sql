-- Add migration script here
CREATE TABLE user_sessions (
    user_id TEXT PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE message(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES user_sessions(user_id) ON DELETE CASCADE,
    user_text TEXT,
    ai_response TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
)

CREATE INDEX idx_message_user_recent ON message (user_id, id DESC);

CREATE TABLE Memory(

)
