import { describe, expect, it } from "vitest";
import { loginPath, safeReturnTo } from "./returnTo";

describe("loginPath", () => {
  it("adds no target for home", () => {
    expect(loginPath("/")).toBe("/login");
  });

  it("encodes the target with its query and hash", () => {
    expect(loginPath("/listings/42?tab=1#owner")).toBe(
      "/login?redirect=%2Flistings%2F42%3Ftab%3D1%23owner"
    );
  });
});

describe("safeReturnTo", () => {
  it("keeps an in-app path with its query and hash", () => {
    expect(safeReturnTo("/listings/42?tab=1#owner")).toBe("/listings/42?tab=1#owner");
  });

  it("round-trips what loginPath produced", () => {
    const target = new URLSearchParams(loginPath("/search?city=Yaoundé").split("?")[1]).get(
      "redirect"
    );
    expect(safeReturnTo(target)).toBe("/search?city=Yaound%C3%A9");
  });

  it.each([
    ["missing", null],
    ["empty", ""],
    ["an absolute URL", "https://example.com/listings"],
    ["a protocol-relative URL", "//example.com"],
    ["a backslash host", "/\\example.com"],
    ["a tab hiding a second slash", "/\t/example.com"],
    ["an unparsable host", "//["],
    ["an unparsable backslash host", "/\\["],
    ["a javascript: URL", "javascript:alert(1)"],
    ["a relative path", "listings/42"],
    ["the sign-in page", "/login"],
    ["the sign-in page with its own target", "/login?redirect=%2Fprofile"],
    ["the sign-in page in another case with a trailing slash", "/LOGIN/"],
  ])("sends %s home", (_label, raw) => {
    expect(safeReturnTo(raw)).toBe("/");
  });

  it("keeps a page whose path only starts with login", () => {
    expect(safeReturnTo("/login-help")).toBe("/login-help");
  });
});
