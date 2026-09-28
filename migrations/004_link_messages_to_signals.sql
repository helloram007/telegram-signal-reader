ALTER TABLE telegram_messages
ADD COLUMN trading_signal_id BIGINT;

ALTER TABLE telegram_messages
ADD CONSTRAINT fk_message_trading_signal
FOREIGN KEY (trading_signal_id)
REFERENCES trading_signals (id);
