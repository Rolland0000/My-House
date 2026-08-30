import { useQuery } from "@tanstack/react-query";
import { getMe, profileQueryKey } from "../api";

export function useProfile() {
  return useQuery({
    queryKey: profileQueryKey,
    queryFn: getMe,
    select: ({ data }) => data,
  });
}
