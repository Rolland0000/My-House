import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { listOwnerListings, type ListOwnerListingsParams } from "../api";

export function useOwnerListings(params: ListOwnerListingsParams) {
  return useQuery({
    queryKey: ["owner-listings", params],
    queryFn: () => listOwnerListings(params),
    placeholderData: keepPreviousData,
  });
}
