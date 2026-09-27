import { useQuery } from "@tanstack/react-query";
import { getOwnerRequestStatus, ownerRequestStatusQueryKey } from "../api";

export function useOwnerRequestStatus(options?: { enabled?: boolean }) {
  return useQuery({
    queryKey: ownerRequestStatusQueryKey,
    queryFn: getOwnerRequestStatus,
    select: ({ data }) => data,
    retry: false,
    enabled: options?.enabled ?? true,
  });
}
