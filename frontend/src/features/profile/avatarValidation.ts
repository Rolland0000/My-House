import { ApiError } from "../../shared/api/client";
import { ACCEPTED_AVATAR_TYPES, MAX_AVATAR_SIZE_BYTES } from "../../shared/api/constants";

export const FORMAT_MESSAGE = "Unsupported format. Choose a JPEG, PNG, or WebP image.";
export const SIZE_MESSAGE = "Image too large. Maximum size is 5 MB.";
export const GENERIC_ERROR_MESSAGE = "Upload failed. Please try again.";

/** Pre-checks that spare an obviously doomed round-trip. The server re-runs
 *  both on the actual bytes, so a file that slips through here still gets
 *  rejected — see `serverMessage`. */
export function preCheck(file: File): string | null {
  if (!ACCEPTED_AVATAR_TYPES.includes(file.type)) return FORMAT_MESSAGE;
  if (file.size > MAX_AVATAR_SIZE_BYTES) return SIZE_MESSAGE;
  return null;
}

/** Distinct messages per backend error code (`shared/errors.rs`); unmapped
 *  codes get a generic fallback, never the server's raw text. */
export function serverMessage(error: unknown): string {
  if (!(error instanceof ApiError)) return GENERIC_ERROR_MESSAGE;
  if (error.code === "PAYLOAD_TOO_LARGE") return SIZE_MESSAGE;
  if (error.code === "INVALID_FILE") return FORMAT_MESSAGE;
  if (error.code === "BAD_REQUEST") return "Unreadable file. Choose another image.";
  return GENERIC_ERROR_MESSAGE;
}
