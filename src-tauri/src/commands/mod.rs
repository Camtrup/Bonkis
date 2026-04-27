pub mod session;
pub mod node;
pub mod runner;
pub mod tools;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionRow {
    pub id: String,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeRow {
    pub id: String,
    pub session_id: String,
    pub parent_id: Option<String>,
    pub tool: String,
    pub args: String,
    pub working_dir: Option<String>,
    pub status: String,
    pub exit_code: Option<i32>,
    pub summary: Option<String>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub finished_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputRow {
    pub id: i64,
    pub node_id: String,
    pub stream: String,
    pub line: String,
    pub created_at: i64,
}
