import { describe, expect, it } from "vitest";
import { listingVisibility } from "./listingVisibility";

describe("listingVisibility", () => {
  it("returns draft when publishedAt is null and there is no photo", () => {
    expect(listingVisibility({ publishedAt: null, hasPhoto: false })).toBe("draft");
  });

  it("returns draft when publishedAt is null even with a photo", () => {
    expect(listingVisibility({ publishedAt: null, hasPhoto: true })).toBe("draft");
  });

  it("returns hidden when published but there is no photo", () => {
    expect(listingVisibility({ publishedAt: "2026-01-01T00:00:00Z", hasPhoto: false })).toBe(
      "hidden"
    );
  });

  it("returns published when published and there is a photo", () => {
    expect(listingVisibility({ publishedAt: "2026-01-01T00:00:00Z", hasPhoto: true })).toBe(
      "published"
    );
  });
});
