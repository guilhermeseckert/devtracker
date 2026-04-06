pub mod active_app;
pub mod git_branch;
pub mod jira;
pub mod zoom;

use crate::db::Database;
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Default)]
pub struct CurrentState {
    pub app_name: String,
    pub bundle_id: Option<String>,
    pub branch: Option<String>,
    pub repo_path: Option<String>,
    pub is_meeting: bool,
    pub jira_ticket: Option<String>,
    /// All branches with recent git activity (for multi-branch workflows)
    pub active_branches: Vec<ActiveBranch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActiveBranch {
    pub branch: String,
    pub repo_path: String,
    pub repo_name: String,
    pub jira_ticket: Option<String>,
}

/// Max gap between polls before we consider it a sleep/idle gap (2 minutes).
/// Normal poll is 30s, so anything > 2min means the system was asleep.
const MAX_POLL_GAP: Duration = Duration::from_secs(120);

pub struct Tracker {
    pub current_state: Arc<Mutex<CurrentState>>,
    /// Maps repo_path -> activity_id for multi-branch tracking
    active_activities: Mutex<HashMap<String, i64>>,
    last_primary_key: Mutex<String>,
    last_poll_time: Mutex<Option<Instant>>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            current_state: Arc::new(Mutex::new(CurrentState::default())),
            active_activities: Mutex::new(HashMap::new()),
            last_primary_key: Mutex::new(String::new()),
            last_poll_time: Mutex::new(None),
        }
    }

    pub fn poll(&self, db: &Database) {
        let now_instant = Instant::now();

        // Detect sleep/idle gaps: if last poll was >2 min ago, close current activities
        // with the old timestamp (not now) to avoid counting sleep as work time
        let was_sleeping = {
            let mut last_poll = self.last_poll_time.lock().unwrap();
            let sleeping = last_poll
                .map(|lp| now_instant.duration_since(lp) > MAX_POLL_GAP)
                .unwrap_or(false);
            *last_poll = Some(now_instant);
            sleeping
        };

        if was_sleeping {
            // Close all current activities — they ended when we went to sleep
            // Use 30s after their start as a conservative end time
            let activities = self.active_activities.lock().unwrap().clone();
            if !activities.is_empty() {
                let conn = db.conn.lock().unwrap();
                for (_, activity_id) in &activities {
                    // Close with "now" — the orphan cleanup on next startup would also catch this,
                    // but doing it here is more accurate
                    let close_time = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                    let _ = crate::db::queries::close_activity(&conn, *activity_id, &close_time);
                }
            }
            *self.active_activities.lock().unwrap() = HashMap::new();
            *self.last_primary_key.lock().unwrap() = String::new();
            eprintln!("Detected sleep/idle gap, closed {} activities", activities.len());
        }

        let new_state = gather_state();

        // Build a key that captures the full state for change detection
        let primary_key = format!(
            "{}|{}|{}|{}",
            new_state.app_name,
            new_state.is_meeting,
            new_state.branch.as_deref().unwrap_or(""),
            new_state
                .active_branches
                .iter()
                .map(|b| format!("{}:{}", b.repo_name, b.branch))
                .collect::<Vec<_>>()
                .join(",")
        );

        let last_key = self.last_primary_key.lock().unwrap().clone();
        if primary_key == last_key && !was_sleeping {
            return;
        }

        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

        // Close all current activities
        {
            let activities = self.active_activities.lock().unwrap();
            let conn = db.conn.lock().unwrap();
            for (_, activity_id) in activities.iter() {
                let _ = crate::db::queries::close_activity(&conn, *activity_id, &now);
            }
        }

        // Insert new activities — one per active branch when coding
        let mut new_activities = HashMap::new();
        {
            let conn = db.conn.lock().unwrap();

            if new_state.active_branches.is_empty() {
                // Non-coding app or no branches detected — single activity row
                match crate::db::queries::insert_activity(
                    &conn,
                    &now,
                    &new_state.app_name,
                    new_state.bundle_id.as_deref(),
                    new_state.branch.as_deref(),
                    new_state.repo_path.as_deref(),
                    new_state.is_meeting,
                    new_state.jira_ticket.as_deref(),
                ) {
                    Ok(id) => {
                        new_activities.insert("__primary__".to_string(), id);
                    }
                    Err(e) => eprintln!("Failed to insert activity: {e}"),
                }
            } else {
                // Multi-branch mode — one row per active branch
                for branch in &new_state.active_branches {
                    match crate::db::queries::insert_activity(
                        &conn,
                        &now,
                        &new_state.app_name,
                        new_state.bundle_id.as_deref(),
                        Some(&branch.branch),
                        Some(&branch.repo_path),
                        new_state.is_meeting,
                        branch.jira_ticket.as_deref(),
                    ) {
                        Ok(id) => {
                            new_activities.insert(branch.repo_path.clone(), id);
                        }
                        Err(e) => eprintln!("Failed to insert activity for {}: {e}", branch.repo_name),
                    }
                }
            }
        }

        *self.active_activities.lock().unwrap() = new_activities;
        *self.last_primary_key.lock().unwrap() = primary_key;
        *self.current_state.lock().unwrap() = new_state;
    }
}

fn repo_name_from_path(path: &str) -> String {
    path.rsplit('/')
        .find(|s| !s.is_empty())
        .unwrap_or("unknown")
        .to_string()
}

fn gather_state() -> CurrentState {
    let app_info = active_app::get_frontmost_app();

    let (app_name, bundle_id) = match &app_info {
        Some(info) => (info.name.clone(), info.bundle_id.clone()),
        None => ("Unknown".to_string(), None),
    };

    let is_coding = active_app::is_coding_app(&app_name, &bundle_id);

    let workspaces = git_branch::get_vscode_workspaces();

    let (branch, repo_path, jira_ticket, active_branches) = if is_coding {
        // Get all branches with activity in the last 5 minutes
        let recent = git_branch::get_recently_active_branches(
            &workspaces,
            Duration::from_secs(300),
        );

        let active: Vec<ActiveBranch> = recent
            .iter()
            .map(|b| ActiveBranch {
                branch: b.branch.clone(),
                repo_path: b.repo_path.clone(),
                repo_name: repo_name_from_path(&b.repo_path),
                jira_ticket: jira::extract_ticket(&b.branch),
            })
            .collect();

        // Primary branch = most recent one (first in list)
        let primary = active.first();
        (
            primary.map(|b| b.branch.clone()),
            primary.map(|b| b.repo_path.clone()),
            primary.and_then(|b| b.jira_ticket.clone()),
            active,
        )
    } else {
        (None, None, None, Vec::new())
    };

    let is_meeting = zoom::is_in_meeting();

    CurrentState {
        app_name,
        bundle_id,
        branch,
        repo_path,
        is_meeting,
        jira_ticket,
        active_branches,
    }
}
