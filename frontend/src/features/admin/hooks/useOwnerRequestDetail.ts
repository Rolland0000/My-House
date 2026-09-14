import { useQuery } from "@tanstack/react-query";
import { getOwnerRequest, ownerRequestDetailQueryKey } from "../api";

export function useOwnerRequestDetail(id: string | undefined) {
  return useQuery({
    queryKey: ownerRequestDetailQueryKey(id ?? ""),
    queryFn: () => getOwnerRequest(id as string),
    select: ({ data }) => data,
    enabled: Boolean(id),
  });
}
