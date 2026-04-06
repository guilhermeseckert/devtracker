import { format, parseISO } from "date-fns";
import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { Activity } from "@/lib/types";
import { cn } from "@/lib/utils";
import { TagEditor } from "./tag-editor";

interface Props {
  activity: Activity;
  onTagUpdate: (id: number, ticket: string) => void;
}

function getDuration(start: string, end: string | null): string {
  const startDate = parseISO(start);
  const endDate = end ? parseISO(end) : new Date();
  const diffMs = endDate.getTime() - startDate.getTime();
  const minutes = Math.floor(diffMs / 60_000);

  if (minutes < 1) {
    return "<1m";
  }
  if (minutes < 60) {
    return `${minutes}m`;
  }
  const hours = Math.floor(minutes / 60);
  const remainingMins = minutes % 60;
  return remainingMins > 0 ? `${hours}h ${remainingMins}m` : `${hours}h`;
}

function getAccentColor(activity: Activity) {
  if (activity.is_meeting) {
    return "border-l-emerald-500";
  }
  if (activity.app_name === "Code" || activity.bundle_id?.includes("VSCode")) {
    return "border-l-blue-500";
  }
  return "border-l-muted-foreground/40";
}

export function TimeBlock({ activity, onTagUpdate }: Props) {
  const [editing, setEditing] = useState(false);
  const timeStart = format(parseISO(activity.started_at), "HH:mm");
  const timeEnd = activity.ended_at
    ? format(parseISO(activity.ended_at), "HH:mm")
    : "now";
  const duration = getDuration(activity.started_at, activity.ended_at);

  return (
    <div
      className={cn(
        "rounded-md border-l-[3px] bg-card px-3 py-2 transition-colors hover:bg-accent",
        getAccentColor(activity)
      )}
    >
      <div className="flex items-center justify-between gap-2">
        <div className="flex min-w-0 flex-1 items-center gap-2">
          <span className="whitespace-nowrap font-mono text-muted-foreground text-xs">
            {timeStart}–{timeEnd}
          </span>
          <span className="truncate text-sm">{activity.app_name}</span>
          {activity.is_meeting && (
            <Badge
              className="border-emerald-500/30 px-1.5 py-0 text-[10px] text-emerald-400"
              variant="outline"
            >
              meeting
            </Badge>
          )}
        </div>
        <div className="flex items-center gap-2">
          <Tooltip>
            <TooltipTrigger asChild>
              <span className="text-muted-foreground text-xs">{duration}</span>
            </TooltipTrigger>
            <TooltipContent>
              {timeStart} – {timeEnd}
            </TooltipContent>
          </Tooltip>
          {editing ? (
            <TagEditor
              currentTicket={activity.jira_ticket}
              onCancel={() => setEditing(false)}
              onSave={(ticket) => {
                onTagUpdate(activity.id, ticket);
                setEditing(false);
              }}
            />
          ) : (
            <button
              className="cursor-pointer"
              onClick={() => setEditing(true)}
              type="button"
            >
              {activity.jira_ticket ? (
                <Badge
                  className="px-1.5 py-0 text-[10px]"
                  variant={activity.manual_tag ? "default" : "secondary"}
                >
                  {activity.jira_ticket}
                </Badge>
              ) : (
                <Badge
                  className="border-amber-500/30 px-1.5 py-0 text-[10px] text-amber-400"
                  variant="outline"
                >
                  tag
                </Badge>
              )}
            </button>
          )}
        </div>
      </div>
      {activity.branch && (
        <p className="mt-1 truncate font-mono text-muted-foreground text-xs">
          {activity.branch}
        </p>
      )}
    </div>
  );
}
