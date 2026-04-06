use crate::db::Database;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn export_summary(
    from: String,
    to: String,
    db: State<'_, Arc<Database>>,
) -> Result<String, String> {
    let conn = db.conn.lock().unwrap();
    let summaries = crate::db::queries::get_summary(&conn, &from, &to).map_err(|e| e.to_string())?;

    let mut output = String::new();
    output.push_str(&format!(
        "{:<30} {:>8} {:>10}  {}\n",
        "Label", "Hours", "Sessions", "Type"
    ));
    output.push_str(&"-".repeat(58));
    output.push('\n');

    for s in &summaries {
        let hours = s.total_minutes / 60.0;
        let kind = if s.is_ticket { "ticket" } else { "branch/app" };
        output.push_str(&format!(
            "{:<30} {:>8.1} {:>10}  {}\n",
            s.jira_ticket, hours, s.sessions, kind
        ));
    }

    let total_hours: f64 = summaries.iter().map(|s| s.total_minutes / 60.0).sum();
    output.push_str(&"-".repeat(58));
    output.push('\n');
    output.push_str(&format!("{:<30} {:>8.1}\n", "TOTAL", total_hours));

    Ok(output)
}
