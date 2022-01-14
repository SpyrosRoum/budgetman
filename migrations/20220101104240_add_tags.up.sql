CREATE TABLE IF NOT EXISTS tags
(
    id          SERIAL PRIMARY KEY NOT NULL,
    name        VARCHAR            NOT NULL UNIQUE,
    description TEXT,
    "limit"     NUMERIC,
    balance     NUMERIC            NOT NULL DEFAULT '0.0'::numeric,
    user_id     uuid               NOT NULL REFERENCES users (id)
);
