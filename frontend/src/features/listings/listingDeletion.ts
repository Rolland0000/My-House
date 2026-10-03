import { ApiError } from "../../shared/api/client";

/** A 404 on delete means the listing is already gone, so it counts as a success. */
export function isAlreadyDeleted(error: unknown): boolean {
  return error instanceof ApiError && error.status === 404;
}
