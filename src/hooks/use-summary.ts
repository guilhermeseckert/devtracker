import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";

export function useSummary(from: string, to: string) {
  return useQuery({
    queryKey: ["summary", from, to],
    queryFn: () => api.getSummary(from, to),
    refetchInterval: 60_000,
  });
}

export function useRepoSummary(from: string, to: string) {
  return useQuery({
    queryKey: ["repo-summary", from, to],
    queryFn: () => api.getRepoSummary(from, to),
    refetchInterval: 60_000,
  });
}
