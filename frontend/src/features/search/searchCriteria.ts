import {
  AVAILABILITY_BADGE,
  PRICE_MAX,
  typeLabels,
  type ListingStatus,
  type ListingType,
} from "../listings";
import type { SearchParams } from "./api";
import { PRICE_SLIDER, toPriceBounds, type PriceBounds } from "./priceRange";

/** Same limit as the API, counted in characters. */
export const MAX_KEYWORD_CHARS = 100;
/** The API reads `page` as an unsigned 32-bit integer. */
const MAX_PAGE = 2 ** 32 - 1;
const UUID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
const PAGE_PATTERN = /^[1-9]\d*$/;
const PRICE_PATTERN = /^\d+$/;

const LISTING_TYPES = Object.keys(typeLabels) as ListingType[];
const LISTING_STATUSES = Object.keys(AVAILABILITY_BADGE) as ListingStatus[];

export interface SearchCriteria {
  q: string;
  types: ListingType[];
  /** Ticked status boxes; the API gets a status filter only when exactly one is ticked. */
  statuses: ListingStatus[];
  /** Whole XAF; `null` means no bound. */
  priceMin: number | null;
  priceMax: number | null;
  city: string;
  neighborhood: string;
  ownerId: string;
  page: number;
}

export const EMPTY_CRITERIA: SearchCriteria = {
  q: "",
  types: [],
  statuses: [],
  priceMin: null,
  priceMax: null,
  city: "",
  neighborhood: "",
  ownerId: "",
  page: 1,
};

/** Keeps the known values present in `raw`, deduplicated, in `known` order. */
function pickKnown<T extends string>(known: readonly T[], raw: string[]): T[] {
  return known.filter((value) => raw.includes(value));
}

function parsePage(raw: string | null): number {
  if (!raw || !PAGE_PATTERN.test(raw)) return 1;
  const page = Number(raw);
  return page <= MAX_PAGE ? page : 1;
}

function parsePrice(raw: string | null): number | null {
  if (!raw || !PRICE_PATTERN.test(raw)) return null;
  const price = Number(raw);
  return price <= PRICE_MAX ? price : null;
}

/** Reversed bounds are both dropped; the slider ends count as no bound. */
function parsePriceBounds(params: URLSearchParams): PriceBounds {
  const min = parsePrice(params.get("price_min"));
  const max = parsePrice(params.get("price_max"));
  if (min !== null && max !== null && min > max) return { priceMin: null, priceMax: null };
  return toPriceBounds(min ?? PRICE_SLIDER.min, max ?? PRICE_SLIDER.max);
}

/** Reads the criteria from the URL; an invalid value counts as absent. */
export function parseSearchCriteria(params: URLSearchParams): SearchCriteria {
  const q = params.get("q")?.trim() ?? "";
  const ownerId = params.get("owner_id")?.trim() ?? "";
  return {
    q: [...q].length <= MAX_KEYWORD_CHARS ? q : "",
    types: pickKnown(LISTING_TYPES, params.getAll("type")),
    statuses: pickKnown(LISTING_STATUSES, params.getAll("status")),
    ...parsePriceBounds(params),
    city: params.get("city")?.trim() ?? "",
    neighborhood: params.get("neighborhood")?.trim() ?? "",
    ownerId: UUID_PATTERN.test(ownerId) ? ownerId : "",
    page: parsePage(params.get("page")),
  };
}

/** Writes the criteria in a fixed order, leaving out empty values and page 1. */
export function toUrlSearchParams(criteria: SearchCriteria): URLSearchParams {
  const params = new URLSearchParams();
  const q = criteria.q.trim();
  const city = criteria.city.trim();
  const neighborhood = criteria.neighborhood.trim();
  if (q) params.set("q", q);
  for (const type of pickKnown(LISTING_TYPES, criteria.types)) params.append("type", type);
  for (const status of pickKnown(LISTING_STATUSES, criteria.statuses)) {
    params.append("status", status);
  }
  if (criteria.priceMin !== null) params.set("price_min", String(criteria.priceMin));
  if (criteria.priceMax !== null) params.set("price_max", String(criteria.priceMax));
  if (city) params.set("city", city);
  if (neighborhood) params.set("neighborhood", neighborhood);
  if (criteria.ownerId) params.set("owner_id", criteria.ownerId);
  if (criteria.page > 1) params.set("page", String(criteria.page));
  return params;
}

export function toSearchRequest(criteria: SearchCriteria): SearchParams {
  const request: SearchParams = { page: criteria.page };
  const q = criteria.q.trim();
  const city = criteria.city.trim();
  const neighborhood = criteria.neighborhood.trim();
  if (q) request.q = q;
  if (criteria.types.length > 0) request.types = pickKnown(LISTING_TYPES, criteria.types);
  if (criteria.statuses.length === 1) request.status = criteria.statuses[0];
  if (criteria.priceMin !== null) request.priceMin = criteria.priceMin;
  if (criteria.priceMax !== null) request.priceMax = criteria.priceMax;
  if (city) request.city = city;
  if (neighborhood) request.neighborhood = neighborhood;
  if (criteria.ownerId) request.ownerId = criteria.ownerId;
  return request;
}

/** True when anything other than the page narrows the results. */
export function hasSearchCriteria(criteria: SearchCriteria): boolean {
  return toUrlSearchParams({ ...criteria, page: 1 }).size > 0;
}
