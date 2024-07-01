-- Your SQL goes here
CREATE TABLE messages (
  message_id BIGINT PRIMARY KEY,
  guild_id BIGINT NOT NULL,
  channel_id BIGINT NOT NULL,
  author_id BIGINT NOT NULL,
  content VARCHAR(4000),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)