-- Your SQL goes here
CREATE TABLE configuration_settings (
  id SERIAL PRIMARY KEY,
  section TEXT NOT NULL,
  "name" TEXT NOT NULL,
  "value" TEXT NOT NULL
)
