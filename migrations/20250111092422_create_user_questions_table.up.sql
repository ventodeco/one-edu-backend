CREATE TABLE user_questions
(
    uuid             UUID        PRIMARY KEY,
    user_id          BIGINT      NOT NULL,
    exam_uuid        TEXT        NOT NULL,
    status           TEXT,
    question_details JSONB,
    time_limit       INT,
    started_time     TIMESTAMPTZ,
    ended_time       TIMESTAMPTZ,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
