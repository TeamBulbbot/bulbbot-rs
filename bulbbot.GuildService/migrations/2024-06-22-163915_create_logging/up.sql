-- Your SQL goes here
CREATE TABLE logging (
  guilds_id BIGINT PRIMARY KEY REFERENCES guilds(id),
  mod_action BIGINT,
  auto_mod BIGINT,
  message BIGINT,
  role BIGINT,
  member BIGINT,
  channel BIGINT,
  thread BIGINT,
  join_leave BIGINT,
  invite BIGINT,
  banpool BIGINT,
  other BIGINT
);