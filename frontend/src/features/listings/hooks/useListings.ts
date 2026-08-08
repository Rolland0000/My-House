import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { getListing, listListings, type ListListingsParams } from "../api";

export function useListings(params: ListListingsParams) {
  return useQuery({
    queryKey: ["listings", params],
    queryFn: () => listListings(params),
    // Keeps the current page visible while a filter/page change is in
    // flight instead of flashing an empty grid (wireframe MH-13: filter
    // bar stays interactive during loading).
    placeholderData: keepPreviousData,
  });
}

export function useListing(id: string | undefined) {
  return useQuery({
    queryKey: ["listing", id],
    queryFn: () => getListing(id as string),
    enabled: Boolean(id),
    retry: (failureCount, error) => {
      const status = (error as { status?: number }).status;
      if (status === 404) return false;
      return failureCount < 3;
    },
  });
}
