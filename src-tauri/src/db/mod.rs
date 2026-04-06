pub mod queries;
pub mod schema;

use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self, String> {
        let db_path = get_db_path()?;
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create db directory: {e}"))?;
        }
        let conn =
            Connection::open(&db_path).map_err(|e| format!("Failed to open database: {e}"))?;
        schema::run_migrations(&conn).map_err(|e| format!("Failed to run migrations: {e}"))?;

        // Close any orphaned activities from previous sessions (crash/force-quit)
        match queries::close_orphaned_activities(&conn) {
            Ok(count) if count > 0 => {
                eprintln!("Closed {count} orphaned activities from previous session");
            }
            Err(e) => {
                eprintln!("Warning: failed to close orphaned activities: {e}");
            }
            _ => {}
        }

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

fn get_db_path() -> Result<PathBuf, String> {
    let proj_dirs = ProjectDirs::from("com", "devtracker", "DevTracker")
        .ok_or("Failed to determine app data directory")?;
    Ok(proj_dirs.data_dir().join("tracker.db"))
}
