import { describe, expect, it } from "vitest";
import { parsePageParam } from "./pageParam";

describe("parsePageParam", () => {
  it.each([null, "", "abc", "0", "-2", "1.5", "2abc", "4294967296", "99999999999999999999"])(
    "falls back to 1 for %j",
    (value) => {
      expect(parsePageParam(value)).toBe(1);
    }
  );

  it.each([
    ["1", 1],
    ["7", 7],
    ["4294967295", 4294967295],
  ])("parses %j as %i", (value, expected) => {
    expect(parsePageParam(value)).toBe(expected);
  });
});
