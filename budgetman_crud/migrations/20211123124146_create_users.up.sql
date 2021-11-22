-- Add up migration script here
CREATE TABLE IF NOT EXISTS users(
    id TEXT PRIMARY KEY NOT NULL, -- Uuid V4
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL, -- Argon2id algorithm
    admin BOOL NOT NULL DEFAULT FALSE
);
