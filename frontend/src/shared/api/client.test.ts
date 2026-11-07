import { describe, expect, it } from "vitest";
import { ApiError, buildQueryString, isNotFound } from "./client";

describe("isNotFound", () => {
  it("is true only for a 404 from the API", () => {
    expect(isNotFound(new ApiError(404, "USER_NOT_FOUND", "gone"))).toBe(true);
    expect(isNotFound(new ApiError(403, "FORBIDDEN", "no"))).toBe(false);
    expect(isNotFound(new Error("network"))).toBe(false);
  });
});

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
