use crate::db::Database;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn update_activity_ticket(
    id: i64,
    ticket: String,
    db: State<'_, Arc<Database>>,
) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    crate::db::queries::update_ticket(&conn, id, &ticket).map_err(|e| e.to_string())
}
