import { useQuery } from "@tanstack/react-query";
import { getListingContact, listingContactQueryKey } from "../api";

/** Fetches only through `refetch`, on click. The result stays cached until sign-out or a reload. */
export function useListingContact(listingId: string) {
  return useQuery({
    queryKey: listingContactQueryKey(listingId),
    queryFn: () => getListingContact(listingId),
    select: ({ data }) => data,
    enabled: false,
    staleTime: Infinity,
    gcTime: Infinity,
    // A 404 or 429 would come back unchanged, so a failure is not retried.
    retry: false,
  });
}
