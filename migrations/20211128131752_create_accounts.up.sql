CREATE TABLE IF NOT EXISTS accounts
-- Always available_money <= total_money
-- Total money is available_money + money assigned to goals
-- Adhoc accounts get created on the spot for a transaction and they don't have available/total money or descriptions
(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR NOT NULL UNIQUE,
    description     TEXT,
    available_money NUMERIC          DEFAULT '0.0'::numeric,
    total_money     NUMERIC          DEFAULT '0.0'::numeric,
    user_id         uuid    NOT NULL REFERENCES users (id),
    is_adhoc        BOOL    NOT NULL DEFAULT false,

    CONSTRAINT correct_balance CHECK ( available_money <= total_money )
);
