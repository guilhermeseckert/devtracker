import {
  addDays,
  addMonths,
  addWeeks,
  format,
  isSameMonth,
  isSameWeek,
  isToday,
  subDays,
  subMonths,
  subWeeks,
} from "date-fns";
import { Button } from "@/components/ui/button";

interface Props {
  date: Date;
  mode: "day" | "week" | "month";
  onDateChange: (date: Date) => void;
}

export function DateNav({ date, mode, onDateChange }: Props) {
  const goBack = () => {
    if (mode === "month") {
      onDateChange(subMonths(date, 1));
    } else if (mode === "week") {
      onDateChange(subWeeks(date, 1));
    } else {
      onDateChange(subDays(date, 1));
    }
  };

  const goForward = () => {
    if (mode === "month") {
      onDateChange(addMonths(date, 1));
    } else if (mode === "week") {
      onDateChange(addWeeks(date, 1));
    } else {
      onDateChange(addDays(date, 1));
    }
  };

  const goToday = () => onDateChange(new Date());

  const now = new Date();

  let label: string;
  if (mode === "day") {
    label = isToday(date) ? "Today" : format(date, "EEE, MMM d");
  } else if (mode === "week") {
    label = `Week of ${format(date, "MMM d")}`;
  } else {
    label = format(date, "MMMM yyyy");
  }

  let isCurrentPeriod: boolean;
  if (mode === "day") {
    isCurrentPeriod = isToday(date);
  } else if (mode === "week") {
    isCurrentPeriod = isSameWeek(date, now, { weekStartsOn: 1 });
  } else {
    isCurrentPeriod = isSameMonth(date, now);
  }

  return (
    <div className="flex items-center gap-1">
      <Button className="size-7 p-0" onClick={goBack} size="sm" variant="ghost">
        ‹
      </Button>
      <button
        className={`cursor-pointer rounded px-2 py-1 text-xs transition-colors ${
          isCurrentPeriod
            ? "font-medium text-foreground"
            : "text-muted-foreground hover:text-foreground"
        }`}
        onClick={goToday}
        type="button"
      >
        {label}
      </button>
      <Button
        className="size-7 p-0"
        onClick={goForward}
        size="sm"
        variant="ghost"
      >
        ›
      </Button>
    </div>
  );
}
