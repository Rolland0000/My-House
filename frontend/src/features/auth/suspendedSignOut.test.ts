import { describe, expect, it } from "vitest";
import { isSuspendedSignOut, SUSPENDED_SIGN_OUT_STATE } from "./suspendedSignOut";

describe("isSuspendedSignOut", () => {
  it("recognizes the redirect state only", () => {
    expect(isSuspendedSignOut(SUSPENDED_SIGN_OUT_STATE)).toBe(true);
    expect(isSuspendedSignOut(null)).toBe(false);
    expect(isSuspendedSignOut({ from: "/profile" })).toBe(false);
  });
});
