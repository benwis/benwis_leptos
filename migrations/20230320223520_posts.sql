CREATE TABLE IF NOT EXISTS posts (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id    INTEGER NOT NULL,
  title      TEXT NOT NULL,
  excerpt    TEXT,
  content    TEXT NOT NULL,
  tags       TEXT,
  slug       TEXT NOT NULL,
  published  INTEGER DEFAULT 0 NOT NULL,
  preview    INTEGER DEFAULT 0 NOT NULL,
  hero       TEXT,
  publish_date INTEGER,
  links      TEXT,
  created_at INTEGER DEFAULT(unixepoch()) NOT NULL,
  updated_at INTEGER DEFAULT(unixepoch()) NOT NULL,  
  FOREIGN KEY (user_id) REFERENCES users (id)
)STRICT;

CREATE TRIGGER Trg_Post_Updated
AFTER UPDATE ON posts
FOR EACH ROW
BEGIN
    UPDATE posts SET updated_at = unixepoch() WHERE id = OLD.id;
END;
