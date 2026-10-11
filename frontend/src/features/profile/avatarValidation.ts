import { ApiError } from "../../shared/api/client";
import { imageFileProblem, type ImageFileProblem } from "../../shared/utils/imageFileCheck";

export const FORMAT_MESSAGE = "Unsupported format. Choose a JPEG, PNG, or WebP image.";
export const SIZE_MESSAGE = "Image too large. Maximum size is 5 MB.";
export const GENERIC_ERROR_MESSAGE = "Upload failed. Please try again.";

const PROBLEM_MESSAGES: Record<ImageFileProblem, string> = {
  format: FORMAT_MESSAGE,
  size: SIZE_MESSAGE,
};

/** The server can still reject a file that passes here; see `serverMessage`. */
export function preCheck(file: File): string | null {
  const problem = imageFileProblem(file);
  return problem ? PROBLEM_MESSAGES[problem] : null;
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
