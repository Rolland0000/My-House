import type { ListListingsParams, ListingStatus, ListingType } from "./api";

export interface FeedFilterInputs {
  city: string;
  neighborhood: string;
  type: ListingType | "";
  status: ListingStatus | "";
}

export type FeedFilters = Pick<ListListingsParams, "city" | "neighborhood" | "type" | "status">;

/** Trims the inputs and leaves out the empty ones. */
export function toFeedFilters({ city, neighborhood, type, status }: FeedFilterInputs): FeedFilters {
  const filters: FeedFilters = {};
  if (city.trim()) filters.city = city.trim();
  if (neighborhood.trim()) filters.neighborhood = neighborhood.trim();
  if (type) filters.type = type;
  if (status) filters.status = status;
  return filters;
}

export function hasActiveFilters(filters: FeedFilters): boolean {
  return Object.keys(filters).length > 0;
}
