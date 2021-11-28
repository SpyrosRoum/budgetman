CREATE TABLE IF NOT EXISTS accounts
-- Always available_money <= total_money
-- Total money is available_money + money assigned to goals
-- Adhoc accounts get created on the spot for a transaction and they don't have available/total money or descriptions
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    available_money REAL DEFAULT 0,
    total_money REAL DEFAULT 0,
    user_id TEXT NOT NULL,
    is_adhoc BOOL NOT NULL DEFAULT 0,

    FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT correct_balance CHECK ( available_money <= total_money )
);
