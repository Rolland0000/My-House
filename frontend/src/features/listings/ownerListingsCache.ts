import type { ListListingsResult, ListingStatus } from "./api";

/** Returns a copy where only the matching row carries the new status. */
export function withListingStatus(
  result: ListListingsResult | undefined,
  id: string,
  status: ListingStatus
): ListListingsResult | undefined {
  if (!result) return result;
  return {
    ...result,
    data: result.data.map((listing) => (listing.id === id ? { ...listing, status } : listing)),
  };
}
