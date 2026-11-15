import { parsePageParam } from "../../shared/utils/pageParam";
import { isUuid } from "../../shared/utils/uuid";
import { LISTING_STATUSES, type ListingStatus } from "../listings";
import { MAX_SEARCH_CHARS } from "./adminUsersCriteria";
import { ADMIN_VISIBILITY_BADGE } from "./adminListingVisibilityBadge";
import type { AdminListingVisibility, ListAdminListingsParams } from "./api";

export const LISTING_VISIBILITIES = Object.keys(ADMIN_VISIBILITY_BADGE) as AdminListingVisibility[];

export interface AdminListingsCriteria {
  q: string;
  status: ListingStatus | "";
  visibility: AdminListingVisibility | "";
  ownerId: string;
  page: number;
}

function pickKnown<T extends string>(known: readonly T[], raw: string | null): T | "" {
  return known.find((value) => value === raw) ?? "";
}

/** Reads the criteria from the URL; an invalid value counts as absent, as the API would reject it. */
export function parseAdminListingsCriteria(params: URLSearchParams): AdminListingsCriteria {
  const ownerId = params.get("owner_id")?.trim() ?? "";
  const q = params.get("q")?.trim() ?? "";
  return {
    q: [...q].length <= MAX_SEARCH_CHARS ? q : "",
    status: pickKnown(LISTING_STATUSES, params.get("status")),
    visibility: pickKnown(LISTING_VISIBILITIES, params.get("visibility")),
    ownerId: isUuid(ownerId) ? ownerId : "",
    page: parsePageParam(params.get("page")),
  };
}

/** Writes the criteria in a fixed order, leaving out empty values and page 1. */
export function toAdminListingsUrlParams(criteria: AdminListingsCriteria): URLSearchParams {
  const params = new URLSearchParams();
  if (criteria.q) params.set("q", criteria.q);
  if (criteria.status) params.set("status", criteria.status);
  if (criteria.visibility) params.set("visibility", criteria.visibility);
  if (criteria.ownerId) params.set("owner_id", criteria.ownerId);
  if (criteria.page > 1) params.set("page", String(criteria.page));
  return params;
}

export function toAdminListingsRequest(criteria: AdminListingsCriteria): ListAdminListingsParams {
  return {
    q: criteria.q || undefined,
    status: criteria.status || undefined,
    visibility: criteria.visibility || undefined,
    owner_id: criteria.ownerId || undefined,
    page: criteria.page,
  };
}

/** True when anything other than the page narrows the list. */
export function hasAdminListingsFilters(criteria: AdminListingsCriteria): boolean {
  return Boolean(criteria.q || criteria.status || criteria.visibility || criteria.ownerId);
}
