CREATE TABLE users (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT FALSE,
  CHECK(username <> ''),
  CHECK(length(password)>=8)
)
