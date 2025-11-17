CREATE TABLE IF NOT EXISTS users (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    handle          TEXT NOT NULL UNIQUE,
    passhash        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    token           TEXT NOT NULL UNIQUE,
    user_id         TEXT NOT NULL REFERENCES users(id),
    expiry          BLOB NOT NULL,
    last_access     BLOB NOT NULL,
    revoked         INTEGER NOT NULL DEFAULT 0,
    revoked_at      BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS logs (
    id              TEXT NOT NULL UNIQUE PRIMARY KEY,
    action          TEXT NOT NULL
);
