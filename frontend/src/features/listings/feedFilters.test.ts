import { describe, expect, it } from "vitest";
import { hasActiveFilters, toFeedFilters } from "./feedFilters";

const EMPTY_INPUTS = { city: "", neighborhood: "", type: "", status: "" } as const;

describe("toFeedFilters", () => {
  it("returns an empty object when every input is empty or blank", () => {
    expect(toFeedFilters(EMPTY_INPUTS)).toEqual({});
    expect(toFeedFilters({ ...EMPTY_INPUTS, city: "  ", neighborhood: "\t " })).toEqual({});
  });

  it("trims the text inputs", () => {
    expect(toFeedFilters({ ...EMPTY_INPUTS, city: " Yaoundé ", neighborhood: " Bastos" })).toEqual({
      city: "Yaoundé",
      neighborhood: "Bastos",
    });
  });

  it("keeps type and status", () => {
    expect(toFeedFilters({ ...EMPTY_INPUTS, type: "studio", status: "unavailable" })).toEqual({
      type: "studio",
      status: "unavailable",
    });
  });

  it("combines the four filters", () => {
    expect(
      toFeedFilters({
        city: "Douala",
        neighborhood: "Akwa",
        type: "apartment",
        status: "available",
      })
    ).toEqual({ city: "Douala", neighborhood: "Akwa", type: "apartment", status: "available" });
  });
});

describe("hasActiveFilters", () => {
  it("is false without any filter", () => {
    expect(hasActiveFilters(toFeedFilters(EMPTY_INPUTS))).toBe(false);
  });

  it("is true with only a neighborhood", () => {
    expect(hasActiveFilters(toFeedFilters({ ...EMPTY_INPUTS, neighborhood: "Akwa" }))).toBe(true);
  });

  it("is true with only a status", () => {
    expect(hasActiveFilters(toFeedFilters({ ...EMPTY_INPUTS, status: "available" }))).toBe(true);
  });
});
