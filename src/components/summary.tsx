import {
  addMonths,
  addWeeks,
  format,
  startOfMonth,
  startOfWeek,
} from "date-fns";
import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { useRepoSummary, useSummary } from "@/hooks/use-summary";
import { DateNav } from "./date-nav";

type Range = "week" | "month";
type View = "tickets" | "repos";

export function Summary() {
  const [range, setRange] = useState<Range>("month");
  const [view, setView] = useState<View>("tickets");
  const [date, setDate] = useState(new Date());

  const from =
    range === "week"
      ? format(startOfWeek(date, { weekStartsOn: 1 }), "yyyy-MM-dd")
      : format(startOfMonth(date), "yyyy-MM-dd");

  // Use start of NEXT period so the SQL `< to` includes the entire last day
  const to =
    range === "week"
      ? format(
          addWeeks(startOfWeek(date, { weekStartsOn: 1 }), 1),
          "yyyy-MM-dd"
        )
      : format(addMonths(startOfMonth(date), 1), "yyyy-MM-dd");

  return (
    <ScrollArea className="h-full">
      <div className="flex flex-col gap-3 p-3">
        <div className="flex items-center justify-between">
          <DateNav date={date} mode={range} onDateChange={setDate} />
          <div className="flex gap-1">
            <Button
              className="h-7 text-xs"
              onClick={() => setRange("week")}
              size="sm"
              variant={range === "week" ? "default" : "ghost"}
            >
              Week
            </Button>
            <Button
              className="h-7 text-xs"
              onClick={() => setRange("month")}
              size="sm"
              variant={range === "month" ? "default" : "ghost"}
            >
              Month
            </Button>
          </div>
        </div>

        <div className="flex gap-1">
          <Button
            className="h-6 flex-1 text-xs"
            onClick={() => setView("tickets")}
            size="sm"
            variant={view === "tickets" ? "secondary" : "ghost"}
          >
            By Ticket
          </Button>
          <Button
            className="h-6 flex-1 text-xs"
            onClick={() => setView("repos")}
            size="sm"
            variant={view === "repos" ? "secondary" : "ghost"}
          >
            By Repo
          </Button>
        </div>

        {view === "tickets" ? (
          <TicketView from={from} to={to} />
        ) : (
          <RepoView from={from} to={to} />
        )}
      </div>
    </ScrollArea>
  );
}

function TicketView({ from, to }: { from: string; to: string }) {
  const { data: summaries, isLoading } = useSummary(from, to);

  const totalHours = (summaries ?? []).reduce(
    (acc, s) => acc + s.total_minutes / 60,
    0
  );

  if (isLoading) {
    return (
      <p className="py-4 text-center text-muted-foreground text-sm">
        Loading...
      </p>
    );
  }

  if (!summaries?.length) {
    return (
      <p className="py-4 text-center text-muted-foreground text-sm">
        No data for this period
      </p>
    );
  }

  return (
    <>
      {summaries.map((s) => {
        const hours = s.total_minutes / 60;
        const pct = totalHours > 0 ? (hours / totalHours) * 100 : 0;

        return (
          <Card key={s.jira_ticket}>
            <CardHeader className="px-3 pt-3 pb-2">
              <div className="flex items-center justify-between">
                <CardTitle className="flex items-center gap-2 font-mono text-sm">
                  <span className="max-w-[180px] truncate">
                    {s.jira_ticket}
                  </span>
                  {!s.is_ticket && (
                    <Badge
                      className="shrink-0 border-amber-500/30 text-[10px] text-amber-400"
                      variant="outline"
                    >
                      no ticket
                    </Badge>
                  )}
                </CardTitle>
                <span className="font-semibold text-sm tabular-nums">
                  {hours.toFixed(1)}h
                </span>
              </div>
            </CardHeader>
            <CardContent className="px-3 pb-3">
              <Progress className="h-1.5" value={pct} />
              <div className="mt-1.5 flex justify-between">
                <span className="text-muted-foreground text-xs">
                  {s.sessions} sessions
                </span>
                <span className="text-muted-foreground text-xs tabular-nums">
                  {pct.toFixed(0)}%
                </span>
              </div>
            </CardContent>
          </Card>
        );
      })}

      <Separator />

      <div className="flex justify-between px-1">
        <span className="text-muted-foreground text-xs">Total</span>
        <span className="font-semibold text-sm tabular-nums">
          {totalHours.toFixed(1)}h
        </span>
      </div>
    </>
  );
}

function RepoView({ from, to }: { from: string; to: string }) {
  const { data: repos, isLoading } = useRepoSummary(from, to);

  const totalHours = (repos ?? []).reduce(
    (acc, r) => acc + r.total_minutes / 60,
    0
  );

  if (isLoading) {
    return (
      <p className="py-4 text-center text-muted-foreground text-sm">
        Loading...
      </p>
    );
  }

  if (!repos?.length) {
    return (
      <p className="py-4 text-center text-muted-foreground text-sm">
        No repo data for this period
      </p>
    );
  }

  return (
    <>
      {repos.map((repo) => {
        const hours = repo.total_minutes / 60;
        const pct = totalHours > 0 ? (hours / totalHours) * 100 : 0;

        return (
          <Card key={repo.repo_name}>
            <CardHeader className="px-3 pt-3 pb-2">
              <div className="flex items-center justify-between">
                <CardTitle className="font-medium text-sm">
                  {repo.repo_name}
                </CardTitle>
                <span className="font-semibold text-sm tabular-nums">
                  {hours.toFixed(1)}h
                </span>
              </div>
            </CardHeader>
            <CardContent className="flex flex-col gap-2 px-3 pb-3">
              <Progress className="h-1.5" value={pct} />

              {repo.tickets.length > 0 && (
                <div className="flex flex-wrap gap-1">
                  {repo.tickets.map((t) => (
                    <Badge className="text-[10px]" key={t} variant="secondary">
                      {t}
                    </Badge>
                  ))}
                </div>
              )}

              <div className="flex flex-col gap-1">
                {repo.branches.map((b) => {
                  const branchHours = b.total_minutes / 60;
                  return (
                    <div
                      className="flex items-center justify-between"
                      key={b.branch}
                    >
                      <div className="flex min-w-0 items-center gap-1.5">
                        <span className="max-w-[160px] truncate font-mono text-muted-foreground text-xs">
                          {b.branch}
                        </span>
                        {b.jira_ticket && (
                          <Badge
                            className="h-4 shrink-0 px-1 py-0 text-[9px]"
                            variant="outline"
                          >
                            {b.jira_ticket}
                          </Badge>
                        )}
                      </div>
                      <span className="text-muted-foreground text-xs tabular-nums">
                        {branchHours.toFixed(1)}h
                      </span>
                    </div>
                  );
                })}
              </div>

              <div className="flex justify-between">
                <span className="text-muted-foreground text-xs">
                  {repo.sessions} sessions
                </span>
                <span className="text-muted-foreground text-xs tabular-nums">
                  {pct.toFixed(0)}%
                </span>
              </div>
            </CardContent>
          </Card>
        );
      })}

      <Separator />

      <div className="flex justify-between px-1">
        <span className="text-muted-foreground text-xs">Total</span>
        <span className="font-semibold text-sm tabular-nums">
          {totalHours.toFixed(1)}h
        </span>
      </div>
    </>
  );
}
