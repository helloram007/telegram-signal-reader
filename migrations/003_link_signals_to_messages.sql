ALTER TABLE trading_signals
ADD CONSTRAINT fk_trading_signal_message
FOREIGN KEY (channel_id, message_id)
REFERENCES telegram_messages (channel_id, message_id);
