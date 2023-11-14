CREATE TABLE IF NOT EXISTS users (
  id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  username   TEXT NOT NULL UNIQUE,
  display_name TEXT NOT NULL,
  password   TEXT NOT NULL,
  created_at INTEGER DEFAULT(unixepoch()),
  updated_at INTEGER DEFAULT(unixepoch()) NOT NULL

)STRICT;

  CREATE TABLE IF NOT EXISTS user_permissions (
      id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
      user_id  INTEGER NOT NULL,
      token    TEXT NOT NULL,
      created_at INTEGER DEFAULT(unixepoch()),
      updated_at INTEGER DEFAULT(unixepoch()) NOT NULL
  )STRICT;

  -- INSERT INTO users (id, anonymous, username, password) 
  -- SELECT 0, true, 'Guest', ''
  -- ON CONFLICT(id) DO UPDATE SET
  --     anonymous = EXCLUDED.anonymous,
  --     username = EXCLUDED.username;

  CREATE TRIGGER Trg_User_Updated
  AFTER UPDATE ON users
  FOR EACH ROW
  BEGIN
      UPDATE users SET updated_at = unixepoch() WHERE id = OLD.id;
  END;

  CREATE TRIGGER Trg_Permission_Updated
  AFTER UPDATE ON user_permissions
  FOR EACH ROW
  BEGIN
      UPDATE user_permissions SET updated_at = unixepoch() WHERE id = OLD.id;
  END;

