use rusqlite::{Connection, Result};
use std::path::Path;

const SCHEMA: &str = "
PRAGMA journal_mode=WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS sessions (
  id         TEXT    PRIMARY KEY,
  name       TEXT    NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS nodes (
  id          TEXT    PRIMARY KEY,
  session_id  TEXT    NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
  parent_id   TEXT    REFERENCES nodes(id),
  tool        TEXT    NOT NULL,
  args        TEXT    NOT NULL,
  working_dir TEXT,
  status      TEXT    NOT NULL DEFAULT 'queued',
  exit_code   INTEGER,
  summary     TEXT,
  created_at  INTEGER NOT NULL,
  started_at  INTEGER,
  finished_at INTEGER
);

CREATE TABLE IF NOT EXISTS node_output (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  node_id    TEXT    NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
  stream     TEXT    NOT NULL DEFAULT 'stdout',
  line       TEXT    NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nodes_session ON nodes(session_id);
CREATE INDEX IF NOT EXISTS idx_nodes_parent  ON nodes(parent_id);
CREATE INDEX IF NOT EXISTS idx_output_node   ON node_output(node_id);
";

pub fn init_db(path: &Path) -> Result<()> {
    let conn = Connection::open(path)?;
    conn.execute_batch(SCHEMA)?;
    Ok(())
}

pub fn open_conn(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode=WAL;")?;
    Ok(conn)
}
