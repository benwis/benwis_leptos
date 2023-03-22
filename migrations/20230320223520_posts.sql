CREATE TABLE IF NOT EXISTS posts (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id    INTEGER NOT NULL,
  title      TEXT NOT NULL,
  excerpt    TEXT,
  content    TEXT NOT NULL,
  tags       TEXT,
  slug       TEXT NOT NULL,
  published  BOOLEAN DEFAULT FALSE NOT NULL,
  preview    BOOLEAN DEFAULT FALSE NOT NULL,
  hero       TEXT,
  publish_date TIMESTAMP,
  links      TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,  

  FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TRIGGER Trg_PostUpdated
AFTER UPDATE ON posts
FOR EACH ROW
BEGIN
    UPDATE posts SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END
