export interface Activity {
  app_name: string;
  branch: string | null;
  bundle_id: string | null;
  ended_at: string | null;
  id: number;
  is_meeting: boolean;
  jira_ticket: string | null;
  manual_tag: boolean;
  notes: string | null;
  repo_path: string | null;
  started_at: string;
}

export interface TicketSummary {
  first_seen: string;
  /** True if this is a real Jira ticket, false if branch name or app name */
  is_ticket: boolean;
  jira_ticket: string;
  last_seen: string;
  sessions: number;
  total_minutes: number;
}

export interface RepoBranch {
  branch: string;
  jira_ticket: string | null;
  total_minutes: number;
}

export interface RepoSummary {
  branches: RepoBranch[];
  repo_name: string;
  sessions: number;
  tickets: string[];
  total_minutes: number;
}

export interface ActiveBranch {
  branch: string;
  jira_ticket: string | null;
  repo_name: string;
  repo_path: string;
}

export interface CurrentStatus {
  active_branches: ActiveBranch[];
  app_name: string;
  branch: string | null;
  bundle_id: string | null;
  is_meeting: boolean;
  jira_ticket: string | null;
  repo_path: string | null;
}
