import { describe, expect, it } from "vitest";
import { readTokenRole } from "./tokenRole";

function base64Url(text: string): string {
  return btoa(text).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

function tokenWithPayload(payload: string): string {
  return `${base64Url('{"alg":"HS256"}')}.${base64Url(payload)}.signature`;
}

describe("readTokenRole", () => {
  it.each(["seeker", "owner", "admin"] as const)("reads the %s role", (role) => {
    expect(readTokenRole(tokenWithPayload(JSON.stringify({ sub: "id", role })))).toBe(role);
  });

  it("decodes base64url characters and missing padding", () => {
    // This payload's base64 contains "+" and "/" and ends with "==".
    const payload = JSON.stringify({ role: "owner", note: "??>>???x" });
    expect(readTokenRole(tokenWithPayload(payload))).toBe("owner");
  });

  it("returns null for an unknown role", () => {
    expect(readTokenRole(tokenWithPayload(JSON.stringify({ role: "superuser" })))).toBeNull();
  });

  it("returns null when the role claim is missing", () => {
    expect(readTokenRole(tokenWithPayload(JSON.stringify({ sub: "id" })))).toBeNull();
  });

  it("returns null when the payload is not an object", () => {
    expect(readTokenRole(tokenWithPayload("null"))).toBeNull();
  });

  it("returns null with fewer than three segments", () => {
    expect(readTokenRole("header.payload")).toBeNull();
  });

  it("returns null for invalid base64", () => {
    expect(readTokenRole("header.%%%.signature")).toBeNull();
  });

  it("returns null for invalid JSON", () => {
    expect(readTokenRole(tokenWithPayload("{not json"))).toBeNull();
  });
});
