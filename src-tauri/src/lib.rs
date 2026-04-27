mod db;
mod state;
pub mod commands;

use commands::node::{get_session_tree, get_node, get_node_output, kill_node};
use commands::runner::run_tool;
use commands::session::{create_session, list_sessions, delete_session};
use commands::tools::list_available_tools;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Resolve DB path in app data dir
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
            let db_path = app_dir.join("bonkis.db");

            // Initialize schema
            db::init_db(&db_path).expect("Failed to init database");

            // Register state
            app.manage(AppState::new(db_path));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_session,
            list_sessions,
            delete_session,
            get_session_tree,
            get_node,
            get_node_output,
            kill_node,
            run_tool,
            list_available_tools,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
