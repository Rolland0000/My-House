import { describe, expect, it } from "vitest";
import { isUuid } from "./uuid";

const UUID = "3f2b8c1e-9a4d-4e6f-8b2a-1c5d7e9f0a3b";

describe("isUuid", () => {
  it("accepts a UUID in either case", () => {
    expect(isUuid(UUID)).toBe(true);
    expect(isUuid(UUID.toUpperCase())).toBe(true);
  });

  it.each(["", "xyz", `${UUID}x`, UUID.replaceAll("-", "")])("rejects %j", (value) => {
    expect(isUuid(value)).toBe(false);
  });
});
