CREATE TABLE IF NOT EXISTS users
(
    id           BIGSERIAL PRIMARY KEY,
    uuid         UUID        NOT NULL,
    user_name    TEXT        NOT NULL,
    full_name    TEXT        NOT NULL,
    email        TEXT        NOT NULL,
    phone_number TEXT        NOT NULL,
    partner      TEXT        NOT NULL,
    university   TEXT,
    major        TEXT,
    role         TEXT        NOT NULL DEFAULT 'USER',
    password     TEXT        NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_partner_phone_email_username ON users (partner, phone_number, email, user_name);
CREATE INDEX IF NOT EXISTS idx_users_email_partner ON users (email, partner);
CREATE INDEX IF NOT EXISTS idx_users_username_partner ON users (user_name, partner);
CREATE INDEX IF NOT EXISTS idx_users_phone_partner ON users (phone_number, partner);