use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

#[tauri::command]
pub fn list_available_tools() -> Vec<String> {
    let mut tools: HashSet<String> = HashSet::new();

    if let Some(path_var) = env::var_os("PATH") {
        for dir in env::split_paths(&path_var) {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if is_executable(&path) {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            tools.insert(name.to_string());
                        }
                    }
                }
            }
        }
    }

    // Also pick up .py scripts from a scripts_dir (e.g. ~/pentest_scripts)
    if let Some(home) = dirs_home() {
        let scripts_dir = home.join("pentest_scripts");
        if let Ok(entries) = std::fs::read_dir(&scripts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("py") {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        tools.insert(name.to_string());
                    }
                }
            }
        }
    }

    let mut sorted: Vec<String> = tools.into_iter().collect();
    sorted.sort();
    sorted
}

fn is_executable(path: &std::path::Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = path.metadata() {
            return meta.permissions().mode() & 0o111 != 0;
        }
        false
    }
    #[cfg(not(unix))]
    {
        // On Windows, check for executable extensions
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            matches!(ext.to_lowercase().as_str(), "exe" | "bat" | "cmd" | "com")
        } else {
            false
        }
    }
}

fn dirs_home() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        env::var_os("HOME").map(PathBuf::from)
    }
    #[cfg(windows)]
    {
        env::var_os("USERPROFILE").map(PathBuf::from)
    }
}
