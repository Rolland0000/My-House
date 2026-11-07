import { ApiError } from "../../shared/api/client";

/** A 404 means the listing is already gone; the caller decides what that means for its action. */
export function isAlreadyDeleted(error: unknown): boolean {
  return error instanceof ApiError && error.status === 404;
}
