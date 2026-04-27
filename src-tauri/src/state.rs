use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

pub struct AppState {
    pub db_path: PathBuf,
    pub running: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
}

impl AppState {
    pub fn new(db_path: PathBuf) -> Self {
        AppState {
            db_path,
            running: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
