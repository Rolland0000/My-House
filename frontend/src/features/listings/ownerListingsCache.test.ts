import { describe, expect, it } from "vitest";
import type { ListListingsResult, ListingSummary } from "./api";
import { withListingStatus, withoutListing } from "./ownerListingsCache";

function row(id: string): ListingSummary {
  return {
    id,
    title: `Listing ${id}`,
    city: "Abidjan",
    price: 100000,
    status: "available",
    type: "apartment",
    owner: { id: "owner" },
  };
}

function page(): ListListingsResult {
  return {
    data: [row("a"), row("b")],
    pagination: { page: 1, per_page: 20, total: 2, total_pages: 1 },
  };
}

describe("withListingStatus", () => {
  it("changes only the matching row", () => {
    const input = page();
    const output = withListingStatus(input, "a", "unavailable");

    expect(output?.data[0].status).toBe("unavailable");
    expect(output?.data[1]).toBe(input.data[1]);
    expect(input.data[0].status).toBe("available");
  });

  it("leaves the rows unchanged for an unknown id", () => {
    expect(withListingStatus(page(), "missing", "unavailable")?.data).toEqual(page().data);
  });

  it("passes undefined through", () => {
    expect(withListingStatus(undefined, "a", "unavailable")).toBeUndefined();
  });
});

describe("withoutListing", () => {
  it("removes the matching row and keeps the others", () => {
    const input = page();
    const output = withoutListing(input, "a");

    expect(output?.data).toEqual([input.data[1]]);
    expect(output?.data[0]).toBe(input.data[1]);
    expect(input.data).toHaveLength(2);
  });

  it("passes undefined through", () => {
    expect(withoutListing(undefined, "a")).toBeUndefined();
  });
});
