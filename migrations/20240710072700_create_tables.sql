CREATE TABLE IF NOT EXISTS users (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  albumcode     TEXT NOT NULL UNIQUE,
  passcode      TEXT NOT NULL,
  created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS carts (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  items         TEXT NOT NULL
)
