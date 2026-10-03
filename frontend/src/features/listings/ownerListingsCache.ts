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

/** Returns a copy without the matching row. The counts drop with it so that removing the
 *  last row shows the empty state right away. */
export function withoutListing(
  result: ListListingsResult | undefined,
  id: string
): ListListingsResult | undefined {
  if (!result) return result;
  const data = result.data.filter((listing) => listing.id !== id);
  if (data.length === result.data.length) return result;

  const total = Math.max(result.pagination.total - 1, 0);
  const totalPages = Math.ceil(total / result.pagination.per_page);
  return { data, pagination: { ...result.pagination, total, total_pages: totalPages } };
}
