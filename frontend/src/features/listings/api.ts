import { apiDelete, apiGet, apiPatch, apiPost, apiPut, apiUpload } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type ListingSummary = components["schemas"]["ListingSummaryDto"];
export type ListingDetail = components["schemas"]["ListingDetailDto"];
export type ListingType = components["schemas"]["ListingType"];
export type ListingStatus = components["schemas"]["ListingStatus"];
export type ListingStatusDto = components["schemas"]["ListingStatusDto"];
export type PaginationMeta = components["schemas"]["PaginationMeta"];
export type ListingMedia = components["schemas"]["MediaDto"];

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

export function publishListing(id: string): Promise<{ data: ListingDetail }> {
  return apiPost<{ data: ListingDetail }>(`/api/v1/listings/${encodeURIComponent(id)}/publish`);
}

export function deleteListing(id: string): Promise<void> {
  return apiDelete<void>(`/api/v1/listings/${encodeURIComponent(id)}`);
}

export function uploadListingPhoto(listingId: string, file: File): Promise<{ data: ListingMedia }> {
  const formData = new FormData();
  formData.append("listing_id", listingId);
  formData.append("file", file);
  return apiUpload<{ data: ListingMedia }>("/api/v1/media/upload", formData);
}

export function deleteListingPhoto(mediaId: string): Promise<void> {
  return apiDelete<void>(`/api/v1/media/${encodeURIComponent(mediaId)}`);
}

export function setListingCover(
  listingId: string,
  mediaId: string
): Promise<{ data: ListingMedia }> {
  return apiPatch<{ data: ListingMedia }>(
    `/api/v1/listings/${encodeURIComponent(listingId)}/cover`,
    {
      media_id: mediaId,
    }
  );
}

export function reorderListingPhotos(
  listingId: string,
  mediaIds: string[]
): Promise<{ data: ListingMedia[] }> {
  return apiPut<{ data: ListingMedia[] }>(
    `/api/v1/listings/${encodeURIComponent(listingId)}/media/order`,
    { media_ids: mediaIds }
  );
}
