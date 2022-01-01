CREATE TABLE IF NOT EXISTS tags
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL UNIQUE,
    description TEXT,
    "limit"     REAL,
    balance     REAL                NOT NULL DEFAULT 0,
    user_id     TEXT                NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id)
);
