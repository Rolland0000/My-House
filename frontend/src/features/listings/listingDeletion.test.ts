import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import { isAlreadyDeleted } from "./listingDeletion";

describe("isAlreadyDeleted", () => {
  it("is true for a 404", () => {
    expect(isAlreadyDeleted(new ApiError(404, "LISTING_NOT_FOUND", "not found"))).toBe(true);
  });

  it.each([403, 500])("is false for a %i", (status) => {
    expect(isAlreadyDeleted(new ApiError(status, "ERROR", "failed"))).toBe(false);
  });

  it("is false for a network error or no error", () => {
    expect(isAlreadyDeleted(new TypeError("Failed to fetch"))).toBe(false);
    expect(isAlreadyDeleted(undefined)).toBe(false);
  });
});
