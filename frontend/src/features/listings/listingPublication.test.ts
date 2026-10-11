import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import { isPhotoRequired } from "./listingPublication";

describe("isPhotoRequired", () => {
  it("is true for a 409 PHOTO_REQUIRED", () => {
    expect(isPhotoRequired(new ApiError(409, "PHOTO_REQUIRED", "no photo"))).toBe(true);
  });

  it.each([
    [409, "MEDIA_ORDER_CONFLICT"],
    [404, "LISTING_NOT_FOUND"],
    [500, "INTERNAL_ERROR"],
  ])("is false for a %i %s", (status, code) => {
    expect(isPhotoRequired(new ApiError(status, code, "failed"))).toBe(false);
  });

  it("is false for a network error or no error", () => {
    expect(isPhotoRequired(new TypeError("Failed to fetch"))).toBe(false);
    expect(isPhotoRequired(undefined)).toBe(false);
  });
});
