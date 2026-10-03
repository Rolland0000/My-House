import { ApiError } from "../../shared/api/client";
import type { ListingDetail, ListingRequest, ListingType } from "./api";

// Bounds mirroring `backend/src/modules/listings/service.rs` — the server
// stays the authority; these only spare the user a round-trip.
export const TITLE_MIN_LENGTH = 5;
export const TITLE_MAX_LENGTH = 120;
export const DESCRIPTION_MIN_LENGTH = 20;
export const DESCRIPTION_MAX_LENGTH = 2000;
export const PRICE_MIN = 1;
export const PRICE_MAX = 9_999_999_999;
export const PLACE_NAME_MAX_LENGTH = 100;
export const SURFACE_M2_MIN = 1;
export const SURFACE_M2_MAX = 100_000;
export const ROOMS_MIN = 0;
export const ROOMS_MAX = 100;

export interface ListingFormValues {
  title: string;
  description: string;
  type: ListingType | "";
  price: number;
  surfaceM2: number;
  rooms: number;
  city: string;
  neighborhood: string;
}

const FORM_FIELD_NAMES = new Set<keyof ListingFormValues>([
  "title",
  "description",
  "type",
  "price",
  "surfaceM2",
  "rooms",
  "city",
  "neighborhood",
]);

// The only server field name (`shared/errors.rs` `FieldError.field`) that
// isn't already a valid `ListingFormValues` key as-is.
const SERVER_FIELD_OVERRIDES: Partial<Record<string, keyof ListingFormValues>> = {
  surface_m2: "surfaceM2",
};

/** Maps a 422's server field name back onto this form's field name, for
 *  `setError`. A name the form doesn't have returns `undefined` so it only
 *  shows up in the summary banner. */
export function serverFieldToFormField(field: string): keyof ListingFormValues | undefined {
  const override = SERVER_FIELD_OVERRIDES[field];
  if (override) return override;
  return FORM_FIELD_NAMES.has(field as keyof ListingFormValues)
    ? (field as keyof ListingFormValues)
    : undefined;
}

/** Shared by the three optional-or-required integer fields (price, surface,
 *  rooms): accepts an empty (`NaN`) value, rejects a non-integer one. */
export function integerFieldRule(min: number, max: number, unit = "") {
  return {
    valueAsNumber: true,
    validate: (value: number) =>
      Number.isNaN(value) || Number.isInteger(value) || "Must be a whole number.",
    min: { value: min, message: `At least ${min}${unit}.` },
    max: { value: max, message: `At most ${max}${unit}.` },
  };
}

/** Title and description: required, and bounded on the trimmed length in
 *  characters, as the backend counts it (not UTF-16 units). */
export function trimmedTextFieldRule(label: string, min: number, max: number) {
  return {
    required: `${label} is required.`,
    validate: (value: string) => {
      const length = [...value.trim()].length;
      if (length === 0) return `${label} is required.`;
      if (length < min) return `${min} characters minimum.`;
      if (length > max) return `${max} characters maximum.`;
      return true;
    },
  };
}

/** Shared by city and neighborhood: required, bounded, and rejects a value
 *  that normalizes down to nothing (e.g. spaces only). */
export function placeNameFieldRule(label: string) {
  return {
    required: `${label} is required.`,
    maxLength: {
      value: PLACE_NAME_MAX_LENGTH,
      message: `${PLACE_NAME_MAX_LENGTH} characters maximum.`,
    },
    validate: (value: string) => normalizePlaceName(value).length > 0 || `${label} is required.`,
  };
}

/** Same transformation as the backend's `normalize_place_name`: trim, then
 *  collapse any run of whitespace (including tabs, newlines, NBSP) to one space. */
export function normalizePlaceName(raw: string): string {
  return raw.trim().replace(/\s+/g, " ");
}

/** Built only from values that already passed the form's own validation —
 *  `price`, `type`, `city` and `neighborhood` are never empty here. An
 *  optional numeric field left blank reads as `NaN` (react-hook-form's
 *  `valueAsNumber`) and is omitted rather than sent as `NaN`. */
export function toListingPayload(values: ListingFormValues): ListingRequest {
  const payload: ListingRequest = {
    title: values.title.trim(),
    description: values.description.trim(),
    type: values.type as ListingType,
    price: values.price,
    city: normalizePlaceName(values.city),
    neighborhood: normalizePlaceName(values.neighborhood),
  };
  if (Number.isFinite(values.surfaceM2)) payload.surface_m2 = values.surfaceM2;
  if (Number.isFinite(values.rooms)) payload.rooms = values.rooms;
  return payload;
}

/** Prefill values for the edit form. Missing optional numbers become `NaN`,
 *  the same value an empty number input reads as. */
export function listingDetailToFormValues(listing: ListingDetail): ListingFormValues {
  return {
    title: listing.title,
    description: listing.description,
    type: listing.type,
    // The API serializes price as a float; the form only accepts whole numbers.
    price: Math.round(listing.price),
    surfaceM2: listing.surface_m2 ?? NaN,
    rooms: listing.rooms ?? NaN,
    city: listing.city,
    neighborhood: listing.neighborhood ?? "",
  };
}

export const VALIDATION_FAILED_MESSAGE = "Please fix the highlighted fields below.";
export const ROLE_NOT_ACTIVE_MESSAGE =
  "Your owner access hasn't loaded into this session yet. Sign out and back in, then try again.";
export const GENERIC_ERROR_MESSAGE = "Something went wrong. Please try again.";

/** Distinct messages per backend error code (`shared/errors.rs`); an
 *  unmapped code or a non-`ApiError` gets a generic fallback, never the
 *  server's raw text. */
export function requestErrorMessage(error: unknown): string {
  if (!(error instanceof ApiError)) return GENERIC_ERROR_MESSAGE;
  if (error.code === "VALIDATION_FAILED") return VALIDATION_FAILED_MESSAGE;
  if (error.code === "FORBIDDEN") return ROLE_NOT_ACTIVE_MESSAGE;
  return GENERIC_ERROR_MESSAGE;
}
