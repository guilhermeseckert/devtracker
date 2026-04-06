import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useCurrentStatus } from "@/hooks/use-timeline";

export function StatusBar() {
  const { data: status } = useCurrentStatus();

  if (!status) {
    return (
      <div className="border-b px-4 py-3">
        <p className="text-muted-foreground text-xs">Starting tracker...</p>
      </div>
    );
  }

  const hasManyBranches = status.active_branches.length > 1;

  return (
    <div className="flex flex-col gap-2 border-b px-4 py-3">
      <div className="flex items-center gap-3">
        <div className="flex min-w-0 flex-1 items-center gap-2">
          <span className="truncate font-medium text-sm">
            {status.app_name}
          </span>
          {status.branch && !hasManyBranches && (
            <>
              <Separator className="h-4" orientation="vertical" />
              <span className="truncate font-mono text-primary text-xs">
                {status.branch}
              </span>
            </>
          )}
        </div>
        <div className="flex items-center gap-2">
          {status.jira_ticket && !hasManyBranches && (
            <Badge variant="secondary">{status.jira_ticket}</Badge>
          )}
          {hasManyBranches && (
            <Badge variant="secondary">
              {status.active_branches.length} branches
            </Badge>
          )}
          {status.is_meeting && (
            <Badge className="border-emerald-500/30 bg-emerald-500/20 text-emerald-400">
              In Meeting
            </Badge>
          )}
        </div>
      </div>

      {hasManyBranches && (
        <div className="flex flex-wrap gap-1">
          {status.active_branches.map((b) => (
            <Tooltip key={b.repo_path}>
              <TooltipTrigger asChild>
                <div className="flex items-center gap-1 rounded bg-accent px-2 py-0.5">
                  <span className="text-[10px] text-muted-foreground">
                    {b.repo_name}
                  </span>
                  <span className="max-w-[120px] truncate font-mono text-[10px] text-primary">
                    {b.branch}
                  </span>
                  {b.jira_ticket && (
                    <Badge
                      className="h-4 px-1 py-0 text-[9px]"
                      variant="secondary"
                    >
                      {b.jira_ticket}
                    </Badge>
                  )}
                </div>
              </TooltipTrigger>
              <TooltipContent>
                <p className="font-mono text-xs">{b.repo_path}</p>
                <p className="text-xs">{b.branch}</p>
              </TooltipContent>
            </Tooltip>
          ))}
        </div>
      )}
    </div>
  );
}
