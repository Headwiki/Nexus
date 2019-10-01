CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  access_id VARCHAR NOT NULL,
  access_secret VARCHAR NOT NULL,
  access_level SMALLINT NOT NULL DEFAULT 0
)