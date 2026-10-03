import { apiGet, apiPatch, apiPost, apiPut } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type ListingSummary = components["schemas"]["ListingSummaryDto"];
export type ListingDetail = components["schemas"]["ListingDetailDto"];
export type ListingType = components["schemas"]["ListingType"];
export type ListingStatus = components["schemas"]["ListingStatus"];
export type ListingStatusDto = components["schemas"]["ListingStatusDto"];
export type PaginationMeta = components["schemas"]["PaginationMeta"];

export type ListingRequest = components["schemas"]["ListingRequest"];

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

export interface ListOwnerListingsParams {
  page?: number;
  perPage?: number;
}

export function listOwnerListings(
  params: ListOwnerListingsParams = {}
): Promise<ListListingsResult> {
  return apiGet<ListListingsResult>("/api/v1/users/me/listings", {
    page: params.page,
    per_page: params.perPage,
  });
}

export function getListing(id: string): Promise<{ data: ListingDetail }> {
  return apiGet<{ data: ListingDetail }>(`/api/v1/listings/${encodeURIComponent(id)}`);
}

export function createListing(body: ListingRequest): Promise<{ data: ListingDetail }> {
  return apiPost<{ data: ListingDetail }>("/api/v1/listings", body);
}

export function updateListing(id: string, body: ListingRequest): Promise<{ data: ListingDetail }> {
  return apiPut<{ data: ListingDetail }>(`/api/v1/listings/${encodeURIComponent(id)}`, body);
}

export function updateListingStatus(
  id: string,
  status: ListingStatus
): Promise<{ data: ListingStatusDto }> {
  return apiPatch<{ data: ListingStatusDto }>(`/api/v1/listings/${encodeURIComponent(id)}/status`, {
    status,
  });
}
