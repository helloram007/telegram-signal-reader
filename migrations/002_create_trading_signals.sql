CREATE TABLE trading_signals (
    id BIGSERIAL PRIMARY KEY,

    channel_id BIGINT NOT NULL,
    message_id INTEGER NOT NULL,

    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    order_type TEXT NOT NULL,

    entry_low DOUBLE PRECISION NOT NULL,
    entry_high DOUBLE PRECISION,

    stop_loss DOUBLE PRECISION NOT NULL,

    take_profit DOUBLE PRECISION,
    tp1 DOUBLE PRECISION,
    tp2 DOUBLE PRECISION,
    tp3 DOUBLE PRECISION,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (channel_id, message_id)
);