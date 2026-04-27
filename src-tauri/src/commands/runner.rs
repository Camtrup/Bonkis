use crate::commands::NodeRow;
use crate::db::open_conn;
use crate::state::AppState;
use chrono::Utc;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::State;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(serde::Serialize, Clone)]
pub struct NodeOutputEvent {
    pub node_id: String,
    pub stream: String,
    pub line: String,
}

#[derive(serde::Serialize, Clone)]
pub struct NodeStatusEvent {
    pub node_id: String,
    pub status: String,
    pub exit_code: Option<i32>,
    pub summary: Option<String>,
    pub finished_at: Option<i64>,
}

#[tauri::command]
pub async fn run_tool(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    session_id: String,
    parent_id: Option<String>,
    tool: String,
    args: Vec<String>,
    working_dir: Option<String>,
) -> Result<NodeRow, String> {
    let node_id = Uuid::new_v4().to_string();
    let created_at = Utc::now().timestamp_millis();
    let args_json = serde_json::to_string(&args).map_err(|e| e.to_string())?;

    // Insert node as queued
    {
        let db_path = state.db_path.clone();
        let node_id2 = node_id.clone();
        let session_id2 = session_id.clone();
        let parent_id2 = parent_id.clone();
        let tool2 = tool.clone();
        let args_json2 = args_json.clone();
        let working_dir2 = working_dir.clone();
        tokio::task::spawn_blocking(move || {
            let conn = open_conn(&db_path)?;
            conn.execute(
                "INSERT INTO nodes (id, session_id, parent_id, tool, args, working_dir, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'queued', ?7)",
                rusqlite::params![node_id2, session_id2, parent_id2, tool2, args_json2, working_dir2, created_at],
            )?;
            Ok::<_, rusqlite::Error>(())
        })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e: rusqlite::Error| e.to_string())?;
    }

    // Set up kill channel
    let (kill_tx, kill_rx) = oneshot::channel::<()>();
    {
        let mut running = state.running.lock().map_err(|e| e.to_string())?;
        running.insert(node_id.clone(), kill_tx);
    }

    let db_path = state.db_path.clone();
    let running_map = state.running.clone();
    let node_id_task = node_id.clone();
    let tool_task = tool.clone();
    let args_task = args.clone();
    let working_dir_task = working_dir.clone();

    tauri::async_runtime::spawn(async move {
        run_node_task(
            app,
            db_path,
            running_map,
            node_id_task,
            tool_task,
            args_task,
            working_dir_task,
            kill_rx,
        )
        .await;
    });

    Ok(NodeRow {
        id: node_id,
        session_id,
        parent_id,
        tool,
        args: args_json,
        working_dir,
        status: "queued".to_string(),
        exit_code: None,
        summary: None,
        created_at,
        started_at: None,
        finished_at: None,
    })
}

async fn run_node_task(
    app: tauri::AppHandle,
    db_path: PathBuf,
    running_map: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
    node_id: String,
    tool: String,
    args: Vec<String>,
    working_dir: Option<String>,
    kill_rx: oneshot::Receiver<()>,
) {
    use tauri::Emitter;

    let started_at = Utc::now().timestamp_millis();

    // Update to running
    {
        let db_path2 = db_path.clone();
        let node_id2 = node_id.clone();
        let _ = tokio::task::spawn_blocking(move || {
            if let Ok(conn) = open_conn(&db_path2) {
                let _ = conn.execute(
                    "UPDATE nodes SET status = 'running', started_at = ?1 WHERE id = ?2",
                    rusqlite::params![started_at, node_id2],
                );
            }
        })
        .await;
    }

    let _ = app.emit(
        "node_status",
        NodeStatusEvent {
            node_id: node_id.clone(),
            status: "running".to_string(),
            exit_code: None,
            summary: None,
            finished_at: None,
        },
    );

    // Build process
    let mut cmd = tokio::process::Command::new(&tool);
    cmd.args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    if let Some(ref wd) = working_dir {
        cmd.current_dir(wd);
    }

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            let finished_at = Utc::now().timestamp_millis();
            let err_msg = format!("Failed to spawn '{}': {}", tool, e);
            let db_path2 = db_path.clone();
            let node_id2 = node_id.clone();
            let err_msg2 = err_msg.clone();
            let _ = tokio::task::spawn_blocking(move || {
                if let Ok(conn) = open_conn(&db_path2) {
                    let _ = conn.execute(
                        "UPDATE nodes SET status = 'error', exit_code = -1, summary = ?1, finished_at = ?2 WHERE id = ?3",
                        rusqlite::params![err_msg2, finished_at, node_id2],
                    );
                }
            })
            .await;
            let _ = app.emit(
                "node_status",
                NodeStatusEvent {
                    node_id: node_id.clone(),
                    status: "error".to_string(),
                    exit_code: Some(-1),
                    summary: Some(err_msg),
                    finished_at: Some(finished_at),
                },
            );
            running_map.lock().unwrap().remove(&node_id);
            return;
        }
    };

    let stdout = child.stdout.take().expect("stdout");
    let stderr = child.stderr.take().expect("stderr");

    enum Line {
        Out(String),
        Err(String),
        Done,
    }

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Line>(256);

    let tx1 = tx.clone();
    tokio::spawn(async move {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(l)) = lines.next_line().await {
            if tx1.send(Line::Out(l)).await.is_err() {
                break;
            }
        }
        let _ = tx1.send(Line::Done).await;
    });

    let tx2 = tx.clone();
    tokio::spawn(async move {
        let mut lines = BufReader::new(stderr).lines();
        while let Ok(Some(l)) = lines.next_line().await {
            if tx2.send(Line::Err(l)).await.is_err() {
                break;
            }
        }
        let _ = tx2.send(Line::Done).await;
    });
    drop(tx);

    let mut first_stdout_line: Option<String> = None;
    let mut done_count = 0u32;
    let mut killed = false;

    let mut kill_rx = kill_rx;

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    None => break,
                    Some(Line::Done) => {
                        done_count += 1;
                        if done_count >= 2 {
                            break;
                        }
                    }
                    Some(Line::Out(line)) => {
                        let ts = Utc::now().timestamp_millis();
                        if first_stdout_line.is_none() && !line.trim().is_empty() {
                            first_stdout_line = Some(line.clone());
                        }
                        let db_path2 = db_path.clone();
                        let node_id2 = node_id.clone();
                        let line2 = line.clone();
                        let _ = tokio::task::spawn_blocking(move || {
                            if let Ok(conn) = open_conn(&db_path2) {
                                let _ = conn.execute(
                                    "INSERT INTO node_output (node_id, stream, line, created_at) VALUES (?1, ?2, ?3, ?4)",
                                    rusqlite::params![node_id2, "stdout", line2, ts],
                                );
                            }
                        }).await;
                        let _ = app.emit("node_output", NodeOutputEvent {
                            node_id: node_id.clone(),
                            stream: "stdout".to_string(),
                            line,
                        });
                    }
                    Some(Line::Err(line)) => {
                        let ts = Utc::now().timestamp_millis();
                        let db_path2 = db_path.clone();
                        let node_id2 = node_id.clone();
                        let line2 = line.clone();
                        let _ = tokio::task::spawn_blocking(move || {
                            if let Ok(conn) = open_conn(&db_path2) {
                                let _ = conn.execute(
                                    "INSERT INTO node_output (node_id, stream, line, created_at) VALUES (?1, ?2, ?3, ?4)",
                                    rusqlite::params![node_id2, "stderr", line2, ts],
                                );
                            }
                        }).await;
                        let _ = app.emit("node_output", NodeOutputEvent {
                            node_id: node_id.clone(),
                            stream: "stderr".to_string(),
                            line,
                        });
                    }
                }
            }
            _ = &mut kill_rx => {
                let _ = child.kill().await;
                killed = true;
                break;
            }
        }
    }

    // Wait for process exit
    let exit_status = if killed {
        None
    } else {
        child.wait().await.ok()
    };

    let exit_code: Option<i32> = exit_status.and_then(|s| s.code());
    let final_status = if killed {
        "killed"
    } else if exit_code == Some(0) {
        "success"
    } else {
        "error"
    };
    let finished_at = Utc::now().timestamp_millis();
    let summary = first_stdout_line.clone();

    {
        let db_path2 = db_path.clone();
        let node_id2 = node_id.clone();
        let final_status2 = final_status.to_string();
        let summary2 = summary.clone();
        let _ = tokio::task::spawn_blocking(move || {
            if let Ok(conn) = open_conn(&db_path2) {
                let _ = conn.execute(
                    "UPDATE nodes SET status = ?1, exit_code = ?2, summary = ?3, finished_at = ?4 WHERE id = ?5",
                    rusqlite::params![final_status2, exit_code, summary2, finished_at, node_id2],
                );
            }
        })
        .await;
    }

    let _ = app.emit(
        "node_status",
        NodeStatusEvent {
            node_id: node_id.clone(),
            status: final_status.to_string(),
            exit_code,
            summary,
            finished_at: Some(finished_at),
        },
    );

    running_map.lock().unwrap().remove(&node_id);
}
