import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";

export function useTimeline(date: string) {
  return useQuery({
    queryKey: ["timeline", date],
    queryFn: () => api.getTimeline(date),
    refetchInterval: 30_000,
  });
}

export function useCurrentStatus() {
  return useQuery({
    queryKey: ["status"],
    queryFn: () => api.getCurrentStatus(),
    refetchInterval: 10_000,
  });
}
