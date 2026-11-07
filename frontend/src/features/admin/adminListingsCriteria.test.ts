import { describe, expect, it } from "vitest";
import {
  hasAdminListingsFilters,
  parseAdminListingsCriteria,
  toAdminListingsRequest,
  toAdminListingsUrlParams,
  type AdminListingsCriteria,
} from "./adminListingsCriteria";

const OWNER_ID = "3f2b8c1e-9a4d-4e6f-8b2a-1c5d7e9f0a3b";

const EMPTY: AdminListingsCriteria = { status: "", visibility: "", ownerId: "", page: 1 };
const FULL: AdminListingsCriteria = {
  status: "unavailable",
  visibility: "no_photo",
  ownerId: OWNER_ID,
  page: 3,
};

function parse(query: string): AdminListingsCriteria {
  return parseAdminListingsCriteria(new URLSearchParams(query));
}

describe("parseAdminListingsCriteria", () => {
  it("returns the empty criteria for an empty URL", () => {
    expect(parse("")).toEqual(EMPTY);
  });

  it("round-trips through the URL", () => {
    expect(parseAdminListingsCriteria(toAdminListingsUrlParams(FULL))).toEqual(FULL);
  });

  it("reads every visibility the API knows", () => {
    for (const visibility of ["draft", "no_photo", "public", "hidden"]) {
      expect(parse(`visibility=${visibility}`).visibility).toBe(visibility);
    }
  });

  it("ignores an unknown status or visibility", () => {
    expect(parse("status=sold&visibility=secret")).toEqual(EMPTY);
  });

  it("ignores an owner_id that is not a UUID", () => {
    expect(parse("owner_id=xyz").ownerId).toBe("");
    expect(parse(`owner_id=${OWNER_ID}x`).ownerId).toBe("");
    expect(parse(`owner_id=${OWNER_ID}`).ownerId).toBe(OWNER_ID);
  });

  it("falls back to page 1 for an invalid page", () => {
    for (const page of ["0", "-2", "abc", "1.5", "", "4294967296"]) {
      expect(parse(`page=${page}`).page).toBe(1);
    }
    expect(parse("page=4294967295").page).toBe(4294967295);
  });
});

describe("toAdminListingsUrlParams", () => {
  it("leaves out empty values and page 1", () => {
    expect(toAdminListingsUrlParams(EMPTY).toString()).toBe("");
  });

  it("writes keys in a fixed order", () => {
    expect(toAdminListingsUrlParams(FULL).toString()).toBe(
      `status=unavailable&visibility=no_photo&owner_id=${OWNER_ID}&page=3`
    );
  });
});

describe("toAdminListingsRequest", () => {
  it("sends only the page without filters", () => {
    expect(toAdminListingsRequest(EMPTY)).toEqual({ page: 1 });
  });

  it("passes every filter through under its API name", () => {
    expect(toAdminListingsRequest(FULL)).toEqual({
      status: "unavailable",
      visibility: "no_photo",
      owner_id: OWNER_ID,
      page: 3,
    });
  });
});

describe("hasAdminListingsFilters", () => {
  it("ignores the page", () => {
    expect(hasAdminListingsFilters(EMPTY)).toBe(false);
    expect(hasAdminListingsFilters({ ...EMPTY, page: 4 })).toBe(false);
  });

  it("counts each filter", () => {
    expect(hasAdminListingsFilters({ ...EMPTY, status: "available" })).toBe(true);
    expect(hasAdminListingsFilters({ ...EMPTY, visibility: "draft" })).toBe(true);
    expect(hasAdminListingsFilters({ ...EMPTY, ownerId: OWNER_ID })).toBe(true);
  });
});
