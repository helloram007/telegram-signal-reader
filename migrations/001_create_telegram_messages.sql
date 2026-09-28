CREATE TABLE telegram_messages (
    id BIGSERIAL PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    message_id INTEGER NOT NULL,
    message_date TIMESTAMPTZ NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (channel_id, message_id)
);
