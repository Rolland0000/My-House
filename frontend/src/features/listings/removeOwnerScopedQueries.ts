import type { QueryClient } from "@tanstack/react-query";

// removeQueries, not invalidateQueries: an invalidation would still render the
// cached owner view until the refetch resolves.
export function removeOwnerScopedQueries(queryClient: QueryClient) {
  queryClient.removeQueries({ queryKey: ["listing"] });
}
