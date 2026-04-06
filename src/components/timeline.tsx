import { useQueryClient } from "@tanstack/react-query";
import { format } from "date-fns";
import { useState } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { useTimeline } from "@/hooks/use-timeline";
import { api } from "@/lib/api";
import { DateNav } from "./date-nav";
import { TimeBlock } from "./time-block";

export function Timeline() {
  const [date, setDate] = useState(new Date());
  const dateStr = format(date, "yyyy-MM-dd");
  const { data: activities, isLoading } = useTimeline(dateStr);
  const queryClient = useQueryClient();

  const handleTagUpdate = async (id: number, ticket: string) => {
    await api.updateActivityTicket(id, ticket);
    queryClient.invalidateQueries({ queryKey: ["timeline"] });
  };

  if (isLoading) {
    return (
      <div className="flex h-full flex-1 items-center justify-center">
        <p className="text-muted-foreground text-sm">Loading...</p>
      </div>
    );
  }

  return (
    <ScrollArea className="h-full">
      <div className="flex flex-col gap-1 p-3">
        <div className="mb-2 flex items-center justify-between">
          <DateNav date={date} mode="day" onDateChange={setDate} />
          <span className="text-muted-foreground text-xs">
            {activities?.length ?? 0} blocks
          </span>
        </div>

        {activities?.length ? (
          activities.map((activity) => (
            <TimeBlock
              activity={activity}
              key={activity.id}
              onTagUpdate={handleTagUpdate}
            />
          ))
        ) : (
          <div className="flex flex-col items-center justify-center gap-2 py-12">
            <p className="text-muted-foreground text-sm">
              No activity recorded
            </p>
            <p className="text-muted-foreground/60 text-xs">
              Switch between apps and activity will appear here
            </p>
          </div>
        )}
      </div>
    </ScrollArea>
  );
}
