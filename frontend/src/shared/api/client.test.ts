import { describe, expect, it } from "vitest";
import { buildQueryString } from "./client";

describe("buildQueryString", () => {
  it("returns an empty string without params or when every value is undefined", () => {
    expect(buildQueryString()).toBe("");
    expect(buildQueryString({ city: undefined, page: undefined })).toBe("");
  });

  it("keeps scalar values as single keys", () => {
    expect(buildQueryString({ city: "Douala", page: 2, flag: true })).toBe(
      "?city=Douala&page=2&flag=true"
    );
  });

  it("repeats the key for each array item, in order", () => {
    expect(buildQueryString({ type: ["villa", "studio"], page: 1 })).toBe(
      "?type=villa&type=studio&page=1"
    );
  });

  it("leaves out an empty array", () => {
    expect(buildQueryString({ type: [], city: "Douala" })).toBe("?city=Douala");
    expect(buildQueryString({ type: [] })).toBe("");
  });
});
