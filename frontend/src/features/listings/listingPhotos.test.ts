import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import { MAX_IMAGE_SIZE_BYTES } from "../../shared/api/constants";
import {
  FORMAT_MESSAGE,
  GENERIC_UPLOAD_MESSAGE,
  LISTING_GONE_MESSAGE,
  QUOTA_MESSAGE,
  SIZE_MESSAGE,
  freePhotoSlots,
  isCoverRequiredError,
  photoUploadErrorMessage,
  planPhotoSelection,
} from "./listingPhotos";

function image(name: string, bytes = 1024, type = "image/jpeg"): File {
  return new File([new Uint8Array(bytes)], name, { type });
}

describe("freePhotoSlots", () => {
  it("leaves every slot free on an empty listing", () => {
    expect(freePhotoSlots(0, 0)).toBe(5);
  });

  it("counts in-flight uploads against the limit", () => {
    expect(freePhotoSlots(3, 2)).toBe(0);
  });

  it("never goes negative", () => {
    expect(freePhotoSlots(5, 1)).toBe(0);
  });
});

describe("planPhotoSelection", () => {
  it("uploads every valid file when slots allow", () => {
    const files = [image("a.jpg"), image("b.jpg")];
    expect(planPhotoSelection(files, 5)).toEqual({
      rejected: [],
      toUpload: files,
      skippedCount: 0,
    });
  });

  it("keeps the first valid files and counts the rest as skipped", () => {
    const [first, second, third] = [image("a.jpg"), image("b.jpg"), image("c.jpg")];
    const plan = planPhotoSelection([first, second, third], 1);
    expect(plan.toUpload).toEqual([first]);
    expect(plan.skippedCount).toBe(2);
  });

  it("skips every valid file when no slot is free", () => {
    const plan = planPhotoSelection([image("a.jpg")], 0);
    expect(plan.toUpload).toEqual([]);
    expect(plan.skippedCount).toBe(1);
  });

  it("rejects invalid files without letting them take a slot", () => {
    const gif = image("anim.gif", 1024, "image/gif");
    const huge = image("huge.jpg", MAX_IMAGE_SIZE_BYTES + 1);
    const valid = image("ok.png", 1024, "image/png");
    const plan = planPhotoSelection([gif, huge, valid], 1);
    expect(plan.rejected).toEqual([
      { file: gif, problem: "format" },
      { file: huge, problem: "size" },
    ]);
    expect(plan.toUpload).toEqual([valid]);
    expect(plan.skippedCount).toBe(0);
  });

  it("returns an empty plan for an empty selection", () => {
    expect(planPhotoSelection([], 5)).toEqual({ rejected: [], toUpload: [], skippedCount: 0 });
  });
});

describe("photoUploadErrorMessage", () => {
  it("maps INVALID_FILE to the format message", () => {
    expect(photoUploadErrorMessage(new ApiError(422, "INVALID_FILE", "x"))).toBe(FORMAT_MESSAGE);
  });

  it("maps a 413 to the size message, with or without an error code", () => {
    expect(photoUploadErrorMessage(new ApiError(413, "PAYLOAD_TOO_LARGE", "x"))).toBe(SIZE_MESSAGE);
    expect(photoUploadErrorMessage(new ApiError(413, "UNKNOWN_ERROR", "x"))).toBe(SIZE_MESSAGE);
  });

  it("maps MEDIA_QUOTA_EXCEEDED to the quota message", () => {
    expect(photoUploadErrorMessage(new ApiError(409, "MEDIA_QUOTA_EXCEEDED", "x"))).toBe(
      QUOTA_MESSAGE
    );
  });

  it("maps a 404 to the listing-gone message", () => {
    expect(photoUploadErrorMessage(new ApiError(404, "LISTING_NOT_FOUND", "x"))).toBe(
      LISTING_GONE_MESSAGE
    );
  });

  it("falls back to a generic message for a network error or an unmapped code", () => {
    expect(photoUploadErrorMessage(new TypeError("Failed to fetch"))).toBe(GENERIC_UPLOAD_MESSAGE);
    expect(photoUploadErrorMessage(new ApiError(500, "INTERNAL_ERROR", "raw text"))).toBe(
      GENERIC_UPLOAD_MESSAGE
    );
  });
});

describe("isCoverRequiredError", () => {
  it("recognizes COVER_PHOTO_REQUIRED", () => {
    expect(isCoverRequiredError(new ApiError(409, "COVER_PHOTO_REQUIRED", "x"))).toBe(true);
  });

  it("ignores other errors", () => {
    expect(isCoverRequiredError(new ApiError(409, "MEDIA_QUOTA_EXCEEDED", "x"))).toBe(false);
    expect(isCoverRequiredError(new ApiError(404, "MEDIA_NOT_FOUND", "x"))).toBe(false);
    expect(isCoverRequiredError(new Error("x"))).toBe(false);
  });
});
