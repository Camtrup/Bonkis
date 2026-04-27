use crate::commands::{NodeRow, OutputRow};
use crate::db::open_conn;
use crate::state::AppState;
use tauri::State;

fn query_node(conn: &rusqlite::Connection, id: &str) -> Result<NodeRow, String> {
    conn.query_row(
        "SELECT id, session_id, parent_id, tool, args, working_dir, status, exit_code, summary, created_at, started_at, finished_at FROM nodes WHERE id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(NodeRow {
                id: row.get(0)?,
                session_id: row.get(1)?,
                parent_id: row.get(2)?,
                tool: row.get(3)?,
                args: row.get(4)?,
                working_dir: row.get(5)?,
                status: row.get(6)?,
                exit_code: row.get(7)?,
                summary: row.get(8)?,
                created_at: row.get(9)?,
                started_at: row.get(10)?,
                finished_at: row.get(11)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_session_tree(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<Vec<NodeRow>, String> {
    let conn = open_conn(&state.db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, session_id, parent_id, tool, args, working_dir, status, exit_code, summary, created_at, started_at, finished_at FROM nodes WHERE session_id = ?1 ORDER BY created_at ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(rusqlite::params![session_id], |row| {
            Ok(NodeRow {
                id: row.get(0)?,
                session_id: row.get(1)?,
                parent_id: row.get(2)?,
                tool: row.get(3)?,
                args: row.get(4)?,
                working_dir: row.get(5)?,
                status: row.get(6)?,
                exit_code: row.get(7)?,
                summary: row.get(8)?,
                created_at: row.get(9)?,
                started_at: row.get(10)?,
                finished_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn get_node(state: State<'_, AppState>, id: String) -> Result<NodeRow, String> {
    let conn = open_conn(&state.db_path).map_err(|e| e.to_string())?;
    query_node(&conn, &id)
}

#[tauri::command]
pub fn get_node_output(
    state: State<'_, AppState>,
    node_id: String,
) -> Result<Vec<OutputRow>, String> {
    let conn = open_conn(&state.db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, node_id, stream, line, created_at FROM node_output WHERE node_id = ?1 ORDER BY id ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(rusqlite::params![node_id], |row| {
            Ok(OutputRow {
                id: row.get(0)?,
                node_id: row.get(1)?,
                stream: row.get(2)?,
                line: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn kill_node(state: State<'_, AppState>, node_id: String) -> Result<bool, String> {
    let mut running = state.running.lock().map_err(|e| e.to_string())?;
    if let Some(sender) = running.remove(&node_id) {
        let _ = sender.send(());
        Ok(true)
    } else {
        Ok(false)
    }
}
