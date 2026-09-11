import { useQuery } from "@tanstack/react-query";
import { getMe, profileQueryKey } from "../api";

export function useProfile(options?: { enabled?: boolean }) {
  return useQuery({
    queryKey: profileQueryKey,
    queryFn: getMe,
    select: ({ data }) => data,
    enabled: options?.enabled ?? true,
  });
}
