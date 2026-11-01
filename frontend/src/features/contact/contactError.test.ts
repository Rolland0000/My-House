import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import { contactErrorKind } from "./contactError";

describe("contactErrorKind", () => {
  it("maps a 404 to not-found", () => {
    expect(contactErrorKind(new ApiError(404, "LISTING_NOT_FOUND", "not found"))).toBe("not-found");
  });

  it("maps a 429 to rate-limited", () => {
    expect(contactErrorKind(new ApiError(429, "RATE_LIMITED", "slow down"))).toBe("rate-limited");
  });

  it("maps a 401 to unauthorized", () => {
    expect(contactErrorKind(new ApiError(401, "UNAUTHORIZED", "sign in"))).toBe("unauthorized");
  });

  it("maps any other ApiError to other", () => {
    expect(contactErrorKind(new ApiError(500, "INTERNAL_ERROR", "boom"))).toBe("other");
  });

  it("maps a non-ApiError value to other", () => {
    expect(contactErrorKind(new Error("network down"))).toBe("other");
  });
});
