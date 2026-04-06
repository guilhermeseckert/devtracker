use rusqlite::{Connection, Result};

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS activities (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at  TEXT NOT NULL,
            ended_at    TEXT,
            app_name    TEXT NOT NULL,
            bundle_id   TEXT,
            branch      TEXT,
            repo_path   TEXT,
            is_meeting  INTEGER NOT NULL DEFAULT 0,
            jira_ticket TEXT,
            manual_tag  INTEGER NOT NULL DEFAULT 0,
            notes       TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_activities_started ON activities(started_at);
        CREATE INDEX IF NOT EXISTS idx_activities_ticket ON activities(jira_ticket);
        ",
    )?;
    Ok(())
}
