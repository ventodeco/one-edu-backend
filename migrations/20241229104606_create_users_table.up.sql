CREATE TABLE users
(
    id           BIGSERIAL PRIMARY KEY,
    uuid         UUID        NOT NULL,
    user_name    TEXT        NOT NULL UNIQUE,
    full_name    TEXT        NOT NULL,
    email        TEXT        NOT NULL UNIQUE,
    phone_number TEXT UNIQUE,
    university   TEXT,
    major        TEXT,
    role         TEXT        NOT NULL DEFAULT 'USER',
    password     TEXT        NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
