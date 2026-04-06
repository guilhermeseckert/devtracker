use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
pub struct BranchInfo {
    pub branch: String,
    pub repo_path: String,
}

pub fn get_current_branch(repo_path: &Path) -> Option<BranchInfo> {
    let head_path = repo_path.join(".git/HEAD");
    let content = fs::read_to_string(&head_path).ok()?;
    let content = content.trim();

    let branch = if let Some(ref_path) = content.strip_prefix("ref: refs/heads/") {
        ref_path.to_string()
    } else {
        content.chars().take(8).collect()
    };

    Some(BranchInfo {
        branch,
        repo_path: repo_path.to_string_lossy().to_string(),
    })
}

/// Find VS Code recently opened workspaces by reading its state database
pub fn get_vscode_workspaces() -> Vec<PathBuf> {
    let home = dirs_home();
    let state_db = home.join("Library/Application Support/Code/User/globalStorage/state.vscdb");

    if !state_db.exists() {
        return Vec::new();
    }

    match read_vscode_state_db(&state_db) {
        Ok(paths) => paths,
        Err(_) => scan_common_dirs(&home),
    }
}

fn read_vscode_state_db(db_path: &Path) -> Result<Vec<PathBuf>, String> {
    let conn =
        rusqlite::Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT value FROM ItemTable WHERE key = 'history.recentlyOpenedPathsList'")
        .map_err(|e| e.to_string())?;

    let json_str: String = stmt.query_row([], |row| row.get(0)).map_err(|e| e.to_string())?;

    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).map_err(|e| e.to_string())?;

    let mut paths = Vec::new();
    if let Some(entries) = parsed.get("entries").and_then(|e| e.as_array()) {
        for entry in entries {
            if let Some(folder_uri) = entry.get("folderUri").and_then(|f| f.as_str()) {
                if let Some(path) = folder_uri.strip_prefix("file://") {
                    let decoded = urlencoding_decode(path);
                    let p = PathBuf::from(&decoded);
                    if p.join(".git/HEAD").exists() {
                        paths.push(p);
                    }
                }
            }
        }
    }

    Ok(paths)
}

fn urlencoding_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn scan_common_dirs(home: &Path) -> Vec<PathBuf> {
    let candidates = ["Desktop", "Documents", "Projects", "Developer", "Code", "repos"];
    let mut paths = Vec::new();

    for dir_name in &candidates {
        let dir = home.join(dir_name);
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.join(".git/HEAD").exists() {
                    paths.push(path);
                }
            }
        }
    }

    paths
}

/// Get ALL branches that had git activity in the last `window` duration.
/// This catches multi-branch workflows (e.g. Claude Code working on 3-4 repos).
pub fn get_recently_active_branches(
    workspaces: &[PathBuf],
    window: Duration,
) -> Vec<BranchInfo> {
    let now = SystemTime::now();

    let mut branches: Vec<(SystemTime, BranchInfo)> = workspaces
        .iter()
        .filter_map(|p| {
            let head = p.join(".git/HEAD");
            let mtime = fs::metadata(&head).ok()?.modified().ok()?;

            // Only include repos with activity within the window
            if now.duration_since(mtime).unwrap_or(Duration::MAX) <= window {
                let info = get_current_branch(p)?;
                Some((mtime, info))
            } else {
                None
            }
        })
        .collect();

    // Sort by most recent first
    branches.sort_by(|a, b| b.0.cmp(&a.0));
    branches.into_iter().map(|(_, info)| info).collect()
}

fn dirs_home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/Users"))
}

mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
