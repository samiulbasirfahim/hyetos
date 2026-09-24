CREATE TABLE auths (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    users TEXT[] NOT NULL DEFAULT '{}', 
    mail_address TEXT NOT NULL UNIQUE,
    google_refresh_token TEXT NOT NULL,
    scopes TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_auths_users ON auths USING GIN (users);
