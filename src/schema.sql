CREATE TABLE IF NOT EXISTS users (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    handle          TEXT NOT NULL UNIQUE,
    passhash        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    token           TEXT NOT NULL UNIQUE,
    user_id         TEXT NOT NULL REFERENCES users(id),
    expiry          INTEGER NOT NULL,
    last_access     INTEGER NOT NULL,
    revoked         INTEGER NOT NULL DEFAULT 0,
    revoked_at      INTEGER DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS logs (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    action          TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS containers (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    name            TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS rewards (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    name            TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS contributions (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    container       TEXT DEFAULT NULL REFERENCES containers(id),
    amount          INTEGER DEFAULT 0,
    notes           TEXT DEFAULT NULL,
    reward          TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS config (
    -- only one record; instance configuration stored under id 0
    --      id_zero is not a primary key so as to not trigger
    --      sqlite autoincrement making it actually 1 instead
    id_zero                             INTEGER UNIQUE DEFAULT 0,
    default_contribution_amount         INTEGER DEFAULT 500 -- in grosze
);
