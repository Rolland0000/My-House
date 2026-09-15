import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import { reviewErrorMessage } from "./reviewErrorMessage";

describe("reviewErrorMessage", () => {
  it("maps a 409 conflict to the already-decided message", () => {
    const error = new ApiError(409, "OWNER_REQUEST_ALREADY_REVIEWED", "conflict");
    expect(reviewErrorMessage(error)).toBe(
      "This request was already decided by another admin — status refreshed below."
    );
  });

  it("falls back to a generic message for any other ApiError", () => {
    const error = new ApiError(500, "INTERNAL_ERROR", "boom");
    expect(reviewErrorMessage(error)).toBe("Something went wrong. Please try again.");
  });

  it("falls back to a generic message for a non-ApiError value", () => {
    expect(reviewErrorMessage(new Error("network down"))).toBe(
      "Something went wrong. Please try again."
    );
  });
});
