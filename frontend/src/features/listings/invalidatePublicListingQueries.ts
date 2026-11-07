import type { QueryClient } from "@tanstack/react-query";

/** Refreshes the feed, the search results and the listing's detail page. */
export function invalidatePublicListingQueries(queryClient: QueryClient, id: string) {
  return Promise.all([
    queryClient.invalidateQueries({ queryKey: ["listings"] }),
    queryClient.invalidateQueries({ queryKey: ["listing", id] }),
  ]);
}
