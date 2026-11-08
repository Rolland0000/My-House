import { afterEach, describe, expect, it, vi } from "vitest";
import {
  ApiError,
  apiGet,
  buildQueryString,
  isAccountSuspended,
  isNotFound,
  setSuspendedHandler,
  setUnauthorizedHandler,
} from "./client";

describe("isNotFound", () => {
  it("is true only for a 404 from the API", () => {
    expect(isNotFound(new ApiError(404, "USER_NOT_FOUND", "gone"))).toBe(true);
    expect(isNotFound(new ApiError(403, "FORBIDDEN", "no"))).toBe(false);
    expect(isNotFound(new Error("network"))).toBe(false);
  });
});

describe("isAccountSuspended", () => {
  it("is true only for the ACCOUNT_SUSPENDED code", () => {
    expect(isAccountSuspended(new ApiError(401, "ACCOUNT_SUSPENDED", "suspended"))).toBe(true);
    expect(isAccountSuspended(new ApiError(401, "UNAUTHORIZED", "no"))).toBe(false);
    expect(isAccountSuspended(new Error("network"))).toBe(false);
  });
});

describe("a 401 on an API path", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    setUnauthorizedHandler(null);
    setSuspendedHandler(null);
  });

  function respondWith401(code: string) {
    const body = JSON.stringify({ error: { code, message: "no", status: 401 } });
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(new Response(body, { status: 401 })));
  }

  it("reports a suspension without trying to refresh", async () => {
    const onUnauthorized = vi.fn().mockResolvedValue(null);
    const onSuspended = vi.fn();
    setUnauthorizedHandler(onUnauthorized);
    setSuspendedHandler(onSuspended);
    respondWith401("ACCOUNT_SUSPENDED");

    await expect(apiGet("/api/v1/users/me")).rejects.toMatchObject({ code: "ACCOUNT_SUSPENDED" });

    expect(onSuspended).toHaveBeenCalledOnce();
    expect(onUnauthorized).not.toHaveBeenCalled();
  });

  it("tries to refresh on any other code", async () => {
    const onUnauthorized = vi.fn().mockResolvedValue(null);
    const onSuspended = vi.fn();
    setUnauthorizedHandler(onUnauthorized);
    setSuspendedHandler(onSuspended);
    respondWith401("UNAUTHORIZED");

    await expect(apiGet("/api/v1/users/me")).rejects.toMatchObject({ code: "UNAUTHORIZED" });

    expect(onUnauthorized).toHaveBeenCalledOnce();
    expect(onSuspended).not.toHaveBeenCalled();
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
