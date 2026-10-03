import { describe, expect, it } from "vitest";
import { parsePageParam } from "./ownerListingsPage";

describe("parsePageParam", () => {
  it.each([null, "", "abc", "0", "-2", "1.5", "2abc", "99999999999999999999"])(
    "falls back to 1 for %j",
    (value) => {
      expect(parsePageParam(value)).toBe(1);
    }
  );

  it.each([
    ["1", 1],
    ["7", 7],
  ])("parses %j as %i", (value, expected) => {
    expect(parsePageParam(value)).toBe(expected);
  });
});
