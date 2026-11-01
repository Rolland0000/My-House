import { ApiError } from "../../shared/api/client";

type ContactErrorKind = "not-found" | "rate-limited" | "unauthorized" | "other";

export function contactErrorKind(error: unknown): ContactErrorKind {
  if (error instanceof ApiError) {
    if (error.status === 404) return "not-found";
    if (error.status === 429) return "rate-limited";
    if (error.status === 401) return "unauthorized";
  }
  return "other";
}
