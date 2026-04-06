use crate::db::queries::{Activity, RepoSummary, TicketSummary};
use crate::db::Database;
use crate::tracking::{CurrentState, Tracker};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn get_current_status(tracker: State<'_, Arc<Tracker>>) -> CurrentState {
    tracker.current_state.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_timeline(date: String, db: State<'_, Arc<Database>>) -> Result<Vec<Activity>, String> {
    let conn = db.conn.lock().unwrap();
    crate::db::queries::get_timeline(&conn, &date).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_summary(
    from: String,
    to: String,
    db: State<'_, Arc<Database>>,
) -> Result<Vec<TicketSummary>, String> {
    let conn = db.conn.lock().unwrap();
    crate::db::queries::get_summary(&conn, &from, &to).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_repo_summary(
    from: String,
    to: String,
    db: State<'_, Arc<Database>>,
) -> Result<Vec<RepoSummary>, String> {
    let conn = db.conn.lock().unwrap();
    crate::db::queries::get_repo_summary(&conn, &from, &to).map_err(|e| e.to_string())
}
