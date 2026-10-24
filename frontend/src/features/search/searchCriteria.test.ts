import { describe, expect, it } from "vitest";
import {
  EMPTY_CRITERIA,
  hasSearchCriteria,
  parseSearchCriteria,
  toSearchRequest,
  toUrlSearchParams,
  type SearchCriteria,
} from "./searchCriteria";

const OWNER_ID = "3f2b8c1e-9a4d-4e6f-8b2a-1c5d7e9f0a3b";

const FULL_CRITERIA: SearchCriteria = {
  q: "villa piscine",
  types: ["apartment", "villa"],
  statuses: ["available"],
  priceMin: 50_000,
  priceMax: 1_500_000,
  city: "Douala",
  neighborhood: "Bonapriso",
  ownerId: OWNER_ID,
  page: 3,
};

function parse(query: string): SearchCriteria {
  return parseSearchCriteria(new URLSearchParams(query));
}

describe("parseSearchCriteria", () => {
  it("returns the empty criteria for an empty URL", () => {
    expect(parse("")).toEqual(EMPTY_CRITERIA);
  });

  it("round-trips through the URL", () => {
    expect(parseSearchCriteria(toUrlSearchParams(FULL_CRITERIA))).toEqual(FULL_CRITERIA);
  });

  it("ignores unknown types and statuses", () => {
    expect(parse("type=castle&type=studio&status=sold")).toEqual({
      ...EMPTY_CRITERIA,
      types: ["studio"],
    });
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

  it("ignores a keyword over 100 characters, counted in characters", () => {
    expect(parse(`q=${"a".repeat(101)}`).q).toBe("");
    expect(parse(`q=${"é".repeat(100)}`).q).toBe("é".repeat(100));
  });

  it("trims text values", () => {
    expect(parse("q=%20villa%20&city=%20Douala&neighborhood=Bastos%20")).toEqual({
      ...EMPTY_CRITERIA,
      q: "villa",
      city: "Douala",
      neighborhood: "Bastos",
    });
  });

  it("deduplicates types and statuses and puts them in canonical order", () => {
    const criteria = parse(
      "type=villa&type=apartment&type=villa&status=unavailable&status=available"
    );
    expect(criteria.types).toEqual(["apartment", "villa"]);
    expect(criteria.statuses).toEqual(["available", "unavailable"]);
  });

  it("reads whole price bounds, alone or together", () => {
    expect(parse("price_min=7499")).toEqual({ ...EMPTY_CRITERIA, priceMin: 7_499 });
    expect(parse("price_max=1500000")).toEqual({ ...EMPTY_CRITERIA, priceMax: 1_500_000 });
    expect(parse("price_min=200000&price_max=200000")).toEqual({
      ...EMPTY_CRITERIA,
      priceMin: 200_000,
      priceMax: 200_000,
    });
  });

  it("ignores a price that is not a whole amount up to the highest listing price", () => {
    for (const price of ["abc", "-5", "1.5", "", "1e5", "10000000000"]) {
      expect(parse(`price_min=${price}&price_max=${price}`)).toEqual(EMPTY_CRITERIA);
    }
    expect(parse("price_max=9999999999").priceMax).toBe(9_999_999_999);
  });

  it("ignores both bounds when they are reversed", () => {
    expect(parse("price_min=500000&price_max=100000")).toEqual(EMPTY_CRITERIA);
  });

  it("reads the slider ends as no bound", () => {
    expect(parse("price_min=0&price_max=1000000")).toEqual(EMPTY_CRITERIA);
  });
});

describe("toUrlSearchParams", () => {
  it("leaves out empty values and page 1", () => {
    expect(toUrlSearchParams(EMPTY_CRITERIA).toString()).toBe("");
    expect(toUrlSearchParams({ ...EMPTY_CRITERIA, q: "  ", city: " " }).toString()).toBe("");
  });

  it("writes keys in a fixed order whatever the input order", () => {
    expect(toUrlSearchParams({ ...FULL_CRITERIA, types: ["villa", "apartment"] }).toString()).toBe(
      `q=villa+piscine&type=apartment&type=villa&status=available&price_min=50000&price_max=1500000&city=Douala&neighborhood=Bonapriso&owner_id=${OWNER_ID}&page=3`
    );
  });
});

describe("toSearchRequest", () => {
  it("sends only the page without criteria", () => {
    expect(toSearchRequest(EMPTY_CRITERIA)).toEqual({ page: 1 });
  });

  it("sends a status only when exactly one box is ticked", () => {
    expect(toSearchRequest({ ...EMPTY_CRITERIA, statuses: ["unavailable"] }).status).toBe(
      "unavailable"
    );
    expect(toSearchRequest({ ...EMPTY_CRITERIA, statuses: [] })).not.toHaveProperty("status");
    expect(
      toSearchRequest({ ...EMPTY_CRITERIA, statuses: ["available", "unavailable"] })
    ).not.toHaveProperty("status");
  });

  it("trims text values and drops blank ones", () => {
    expect(
      toSearchRequest({ ...EMPTY_CRITERIA, q: " villa ", city: " Douala ", neighborhood: "  " })
    ).toEqual({ page: 1, q: "villa", city: "Douala" });
  });

  it("passes types, prices, owner_id and page through", () => {
    expect(toSearchRequest(FULL_CRITERIA)).toEqual({
      page: 3,
      q: "villa piscine",
      types: ["apartment", "villa"],
      status: "available",
      priceMin: 50_000,
      priceMax: 1_500_000,
      city: "Douala",
      neighborhood: "Bonapriso",
      ownerId: OWNER_ID,
    });
  });
});

describe("hasSearchCriteria", () => {
  it("ignores the page", () => {
    expect(hasSearchCriteria(EMPTY_CRITERIA)).toBe(false);
    expect(hasSearchCriteria({ ...EMPTY_CRITERIA, page: 4 })).toBe(false);
  });

  it("counts any keyword, filter or owner", () => {
    expect(hasSearchCriteria({ ...EMPTY_CRITERIA, q: "villa" })).toBe(true);
    expect(hasSearchCriteria({ ...EMPTY_CRITERIA, statuses: ["available", "unavailable"] })).toBe(
      true
    );
    expect(hasSearchCriteria({ ...EMPTY_CRITERIA, ownerId: OWNER_ID })).toBe(true);
    expect(hasSearchCriteria({ ...EMPTY_CRITERIA, priceMax: 0 })).toBe(true);
  });
});
