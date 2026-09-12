import { ApiError } from "../../shared/api/client";
import {
  ACCEPTED_ID_IMAGE_TYPES,
  ACCEPTED_ID_PDF_TYPE,
  MAX_ID_IMAGE_SIZE_BYTES,
  MAX_ID_PDF_SIZE_BYTES,
} from "../../shared/api/constants";

export const IMAGE_FORMAT_MESSAGE = "Unsupported format. Choose a JPEG, PNG, or WebP image.";
export const IMAGE_SIZE_MESSAGE = "Image too large. Maximum size is 5 MB.";
export const PDF_FORMAT_MESSAGE = "Unsupported format. Choose a PDF file.";
export const PDF_SIZE_MESSAGE = "PDF too large. Maximum size is 3 MB.";
export const GENERIC_ERROR_MESSAGE = "Submission failed. Please try again.";

/** Pre-checks that spare an obviously doomed round-trip. The server re-runs
 *  both on the actual bytes, so a file that slips through here still gets
 *  rejected — see `serverMessage`. */
export function preCheckImage(file: File): string | null {
  if (!ACCEPTED_ID_IMAGE_TYPES.includes(file.type)) return IMAGE_FORMAT_MESSAGE;
  if (file.size > MAX_ID_IMAGE_SIZE_BYTES) return IMAGE_SIZE_MESSAGE;
  return null;
}

export function preCheckPdf(file: File): string | null {
  if (!ACCEPTED_ID_PDF_TYPE.includes(file.type)) return PDF_FORMAT_MESSAGE;
  if (file.size > MAX_ID_PDF_SIZE_BYTES) return PDF_SIZE_MESSAGE;
  return null;
}

/** Distinct messages per backend error code (`shared/errors.rs`); unmapped
 *  codes get a generic fallback, never the server's raw text. */
export function serverMessage(error: unknown): string {
  if (!(error instanceof ApiError)) return GENERIC_ERROR_MESSAGE;
  if (error.code === "OWNER_REQUEST_ALREADY_PENDING") {
    return "You already have a pending request.";
  }
  if (error.code === "INVALID_DOCUMENT") {
    return "Choose exactly two photos (front and back) or one PDF.";
  }
  if (error.code === "PAYLOAD_TOO_LARGE") return "A document exceeds the maximum allowed size.";
  if (error.code === "INVALID_FILE") return "Unsupported document format.";
  if (error.code === "BAD_REQUEST") return "Some fields are missing or invalid.";
  return GENERIC_ERROR_MESSAGE;
}
