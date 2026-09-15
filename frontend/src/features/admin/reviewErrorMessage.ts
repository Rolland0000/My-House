import { ApiError } from "../../shared/api/client";

const GENERIC_ERROR_MESSAGE = "Something went wrong. Please try again.";
const ALREADY_DECIDED_MESSAGE =
  "This request was already decided by another admin — status refreshed below.";

export function reviewErrorMessage(error: unknown): string {
  if (error instanceof ApiError && error.code === "OWNER_REQUEST_ALREADY_REVIEWED") {
    return ALREADY_DECIDED_MESSAGE;
  }
  return GENERIC_ERROR_MESSAGE;
}
