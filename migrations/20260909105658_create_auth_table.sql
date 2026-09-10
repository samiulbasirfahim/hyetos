-- Add migration script here
CREATE TABLE auths (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mail_address VARCHAR(255) UNIQUE NOT NULL,
    refresh_token TEXT NOT NULL,
    scopes TEXT[] NOT NULL, 
    platform_users JSONB NOT NULL DEFAULT '[]'::jsonb
);
