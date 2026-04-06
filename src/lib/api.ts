import { invoke } from "@tauri-apps/api/core";
import type {
  Activity,
  CurrentStatus,
  RepoSummary,
  TicketSummary,
} from "./types";

export const api = {
  getCurrentStatus: () => invoke<CurrentStatus>("get_current_status"),

  getTimeline: (date: string) => invoke<Activity[]>("get_timeline", { date }),

  getSummary: (from: string, to: string) =>
    invoke<TicketSummary[]>("get_summary", { from, to }),

  updateActivityTicket: (id: number, ticket: string) =>
    invoke<void>("update_activity_ticket", { id, ticket }),

  getRepoSummary: (from: string, to: string) =>
    invoke<RepoSummary[]>("get_repo_summary", { from, to }),

  exportSummary: (from: string, to: string) =>
    invoke<string>("export_summary", { from, to }),

  getAutostart: () => invoke<boolean>("get_autostart"),

  setAutostart: (enabled: boolean) =>
    invoke<void>("set_autostart", { enabled }),
};
