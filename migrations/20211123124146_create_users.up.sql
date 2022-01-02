CREATE TABLE IF NOT EXISTS users
(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid
(
),
    username VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL, -- Argon2id algorithm
    "admin" BOOL NOT NULL DEFAULT false
    );
