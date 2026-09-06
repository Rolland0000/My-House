import { describe, expect, it } from "vitest";
import { getPageItems } from "./pagination";

describe("getPageItems", () => {
  it("returns every page when there are few enough pages for no ellipsis", () => {
    expect(getPageItems(1, 5, 1)).toEqual([1, 2, 3, 4, 5]);
  });

  it("shows only a right ellipsis near the start", () => {
    expect(getPageItems(1, 20, 1)).toEqual([1, 2, 3, 4, 5, "ellipsis-end", 20]);
  });

  it("shows only a left ellipsis near the end", () => {
    expect(getPageItems(20, 20, 1)).toEqual([1, "ellipsis-start", 16, 17, 18, 19, 20]);
  });

  it("shows both ellipses in the middle", () => {
    expect(getPageItems(10, 20, 1)).toEqual([1, "ellipsis-start", 9, 10, 11, "ellipsis-end", 20]);
  });

  it("widens the visible range with a larger sibling count", () => {
    expect(getPageItems(10, 20, 2)).toEqual([
      1,
      "ellipsis-start",
      8,
      9,
      10,
      11,
      12,
      "ellipsis-end",
      20,
    ]);
  });
});
