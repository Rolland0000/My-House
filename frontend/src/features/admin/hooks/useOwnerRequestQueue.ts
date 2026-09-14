import { useQuery } from "@tanstack/react-query";
import { listOwnerRequests, ownerRequestQueueQueryKey } from "../api";
import type { ListOwnerRequestsParams } from "../api";

export function useOwnerRequestQueue(params: ListOwnerRequestsParams) {
  return useQuery({
    queryKey: ownerRequestQueueQueryKey(params),
    queryFn: () => listOwnerRequests(params),
  });
}
