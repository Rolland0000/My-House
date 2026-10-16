import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { searchListings, type SearchParams } from "../api";

export function useSearch(params: SearchParams) {
  return useQuery({
    // Under the "listings" prefix, so owner mutations that invalidate the feed refresh results too.
    queryKey: ["listings", "search", params],
    queryFn: () => searchListings(params),
    placeholderData: keepPreviousData,
  });
}
