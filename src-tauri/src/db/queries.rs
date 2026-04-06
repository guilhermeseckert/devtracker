use rusqlite::{params, Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Activity {
    pub id: i64,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub app_name: String,
    pub bundle_id: Option<String>,
    pub branch: Option<String>,
    pub repo_path: Option<String>,
    pub is_meeting: bool,
    pub jira_ticket: Option<String>,
    pub manual_tag: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TicketSummary {
    pub jira_ticket: String,
    pub total_minutes: f64,
    pub sessions: i64,
    pub first_seen: String,
    pub last_seen: String,
    /// True if this row is a real Jira ticket, false if it's a branch/app name
    pub is_ticket: bool,
}

#[derive(Debug, Serialize)]
pub struct RepoSummary {
    pub repo_name: String,
    pub total_minutes: f64,
    pub sessions: i64,
    pub branches: Vec<RepoBranch>,
    pub tickets: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RepoBranch {
    pub branch: String,
    pub total_minutes: f64,
    pub jira_ticket: Option<String>,
}

/// Close orphaned activities (no ended_at) from previous sessions.
/// Sets ended_at to started_at + 30s (one poll interval) to avoid inflating hours.
pub fn close_orphaned_activities(conn: &Connection) -> Result<usize> {
    let count = conn.execute(
        "UPDATE activities
         SET ended_at = datetime(started_at, '+30 seconds')
         WHERE ended_at IS NULL",
        [],
    )?;
    Ok(count)
}

/// Cap any activity duration at max_minutes. This prevents sleep/idle from
/// inflating hours (e.g. laptop lid closed for 2 hours mid-activity).
pub fn cap_activity_duration(conn: &Connection, max_minutes: f64) -> Result<usize> {
    let count = conn.execute(
        "UPDATE activities
         SET ended_at = datetime(started_at, '+' || ?1 || ' minutes')
         WHERE ended_at IS NOT NULL
           AND (julianday(ended_at) - julianday(started_at)) * 24 * 60 > ?1",
        params![max_minutes],
    )?;
    Ok(count)
}

pub fn insert_activity(
    conn: &Connection,
    started_at: &str,
    app_name: &str,
    bundle_id: Option<&str>,
    branch: Option<&str>,
    repo_path: Option<&str>,
    is_meeting: bool,
    jira_ticket: Option<&str>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO activities (started_at, app_name, bundle_id, branch, repo_path, is_meeting, jira_ticket)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            started_at,
            app_name,
            bundle_id,
            branch,
            repo_path,
            is_meeting as i32,
            jira_ticket,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn close_activity(conn: &Connection, id: i64, ended_at: &str) -> Result<()> {
    conn.execute(
        "UPDATE activities SET ended_at = ?1 WHERE id = ?2",
        params![ended_at, id],
    )?;
    Ok(())
}

pub fn get_timeline(conn: &Connection, date: &str) -> Result<Vec<Activity>> {
    let mut stmt = conn.prepare(
        "SELECT id, started_at, ended_at, app_name, bundle_id, branch, repo_path,
                is_meeting, jira_ticket, manual_tag, notes
         FROM activities
         WHERE started_at LIKE ?1 || '%'
            OR (started_at < ?1 AND (ended_at > ?1 OR ended_at IS NULL))
         ORDER BY started_at ASC",
    )?;

    let rows = stmt.query_map(params![date], |row| {
        Ok(Activity {
            id: row.get(0)?,
            started_at: row.get(1)?,
            ended_at: row.get(2)?,
            app_name: row.get(3)?,
            bundle_id: row.get(4)?,
            branch: row.get(5)?,
            repo_path: row.get(6)?,
            is_meeting: row.get::<_, i32>(7)? != 0,
            jira_ticket: row.get(8)?,
            manual_tag: row.get::<_, i32>(9)? != 0,
            notes: row.get(10)?,
        })
    })?;

    rows.collect()
}

pub fn get_summary(conn: &Connection, from: &str, to: &str) -> Result<Vec<TicketSummary>> {
    // Group by jira_ticket when available, otherwise fall back to repo/branch,
    // then app name. repo_name is extracted from repo_path (last path segment).
    let mut stmt = conn.prepare(
        "SELECT
            COALESCE(
                jira_ticket,
                CASE
                    WHEN branch IS NOT NULL AND repo_path IS NOT NULL
                    THEN REPLACE(repo_path, RTRIM(repo_path, REPLACE(repo_path, '/', '')), '') || ' / ' || branch
                    WHEN branch IS NOT NULL
                    THEN branch
                    ELSE app_name
                END
            ) as label,
            SUM(
                (julianday(COALESCE(ended_at, datetime('now'))) - julianday(started_at)) * 24 * 60
            ) as total_minutes,
            COUNT(*) as sessions,
            MIN(started_at) as first_seen,
            MAX(COALESCE(ended_at, started_at)) as last_seen,
            CASE WHEN jira_ticket IS NOT NULL THEN 1 ELSE 0 END as is_ticket
         FROM activities
         WHERE started_at >= ?1 AND started_at < ?2
         GROUP BY label
         ORDER BY total_minutes DESC",
    )?;

    let rows = stmt.query_map(params![from, to], |row| {
        Ok(TicketSummary {
            jira_ticket: row.get(0)?,
            total_minutes: row.get(1)?,
            sessions: row.get(2)?,
            first_seen: row.get(3)?,
            last_seen: row.get(4)?,
            is_ticket: row.get::<_, i32>(5)? != 0,
        })
    })?;

    rows.collect()
}

pub fn get_repo_summary(conn: &Connection, from: &str, to: &str) -> Result<Vec<RepoSummary>> {
    // Get per-branch breakdown grouped by repo
    let mut stmt = conn.prepare(
        "SELECT
            REPLACE(repo_path, RTRIM(repo_path, REPLACE(repo_path, '/', '')), '') as repo_name,
            branch,
            jira_ticket,
            SUM(
                (julianday(COALESCE(ended_at, datetime('now'))) - julianday(started_at)) * 24 * 60
            ) as total_minutes,
            COUNT(*) as sessions
         FROM activities
         WHERE started_at >= ?1 AND started_at < ?2
           AND repo_path IS NOT NULL
         GROUP BY repo_name, branch
         ORDER BY repo_name, total_minutes DESC",
    )?;

    let mut repo_map: std::collections::HashMap<String, RepoSummary> = std::collections::HashMap::new();

    let rows = stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, i64>(4)?,
        ))
    })?;

    for row in rows {
        let (repo_name, branch, ticket, minutes, sessions) = row?;
        let entry = repo_map.entry(repo_name.clone()).or_insert_with(|| RepoSummary {
            repo_name: repo_name.clone(),
            total_minutes: 0.0,
            sessions: 0,
            branches: Vec::new(),
            tickets: Vec::new(),
        });
        entry.total_minutes += minutes;
        entry.sessions += sessions;
        if let Some(ref b) = branch {
            entry.branches.push(RepoBranch {
                branch: b.clone(),
                total_minutes: minutes,
                jira_ticket: ticket.clone(),
            });
        }
        if let Some(ref t) = ticket {
            if !entry.tickets.contains(t) {
                entry.tickets.push(t.clone());
            }
        }
    }

    let mut result: Vec<RepoSummary> = repo_map.into_values().collect();
    result.sort_by(|a, b| b.total_minutes.partial_cmp(&a.total_minutes).unwrap_or(std::cmp::Ordering::Equal));
    Ok(result)
}

pub fn update_ticket(conn: &Connection, id: i64, ticket: &str) -> Result<()> {
    if ticket.is_empty() {
        // Clear the manual tag — revert to auto-detected ticket
        conn.execute(
            "UPDATE activities SET jira_ticket = NULL, manual_tag = 0 WHERE id = ?1",
            params![id],
        )?;
    } else {
        conn.execute(
            "UPDATE activities SET jira_ticket = ?1, manual_tag = 1 WHERE id = ?2",
            params![ticket, id],
        )?;
    }
    Ok(())
}
