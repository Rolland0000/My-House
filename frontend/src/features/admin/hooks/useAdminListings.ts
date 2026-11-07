import { keepPreviousData, useQuery, type QueryClient } from "@tanstack/react-query";
import { invalidatePublicListingQueries } from "../../listings";
import {
  ADMIN_LISTINGS_KEY,
  adminListingsQueryKey,
  listAdminListings,
  type ListAdminListingsParams,
} from "../api";

export const LISTING_GONE_MESSAGE = "This property no longer exists";

export function useAdminListings(params: ListAdminListingsParams) {
  return useQuery({
    queryKey: adminListingsQueryKey(params),
    queryFn: () => listAdminListings(params),
    placeholderData: keepPreviousData,
  });
}

/** Refreshes the back-office list and the public pages after a moderation action. */
export function refreshModeratedListing(queryClient: QueryClient, id: string) {
  return Promise.all([
    queryClient.invalidateQueries({ queryKey: ADMIN_LISTINGS_KEY }),
    invalidatePublicListingQueries(queryClient, id),
  ]);
}
