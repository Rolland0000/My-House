import { apiGet, apiPost } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type ListingSummary = components["schemas"]["ListingSummaryDto"];
export type ListingDetail = components["schemas"]["ListingDetailDto"];
export type ListingType = components["schemas"]["ListingType"];
export type ListingStatus = components["schemas"]["ListingStatus"];
export type PaginationMeta = components["schemas"]["PaginationMeta"];

// The backend endpoint doesn't exist yet, so this isn't in the generated
// `types.ts` — replace with `components["schemas"]["CreateListingRequest"]`
// once it ships and types are regenerated.
export interface CreateListingRequest {
  title: string;
  description: string;
  type: ListingType;
  price: number;
  city: string;
  neighborhood: string;
  surface_m2?: number;
  rooms?: number;
}

export interface ListListingsParams {
  city?: string;
  type?: ListingType;
  ownerId?: string;
  page?: number;
  perPage?: number;
}

export interface ListListingsResult {
  data: ListingSummary[];
  pagination: PaginationMeta;
}

export function listListings(params: ListListingsParams = {}): Promise<ListListingsResult> {
  return apiGet<ListListingsResult>("/api/v1/listings", {
    city: params.city,
    type: params.type,
    owner_id: params.ownerId,
    page: params.page,
    per_page: params.perPage,
  });
}

export function getListing(id: string): Promise<{ data: ListingDetail }> {
  return apiGet<{ data: ListingDetail }>(`/api/v1/listings/${encodeURIComponent(id)}`);
}

export function createListing(body: CreateListingRequest): Promise<{ data: ListingDetail }> {
  return apiPost<{ data: ListingDetail }>("/api/v1/listings", body);
}
