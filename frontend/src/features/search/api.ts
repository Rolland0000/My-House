import { apiGet } from "../../shared/api/client";
import type { ListingStatus, ListingType, ListListingsResult } from "../listings";

export interface SearchParams {
  q?: string;
  ownerId?: string;
  city?: string;
  neighborhood?: string;
  types?: ListingType[];
  status?: ListingStatus;
  page?: number;
  perPage?: number;
}

export function searchListings(params: SearchParams = {}): Promise<ListListingsResult> {
  return apiGet<ListListingsResult>("/api/v1/search", {
    q: params.q,
    owner_id: params.ownerId,
    city: params.city,
    neighborhood: params.neighborhood,
    type: params.types,
    status: params.status,
    page: params.page,
    per_page: params.perPage,
  });
}
