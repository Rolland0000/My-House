import { describe, expect, it } from "vitest";
import { isDisplayableMediaUrl, isRemoteMediaUrl } from "./mediaUrl";

describe("isRemoteMediaUrl", () => {
  it("accepts http and https URLs", () => {
    expect(isRemoteMediaUrl("http://example.com/a.jpg")).toBe(true);
    expect(isRemoteMediaUrl("https://example.com/a.jpg")).toBe(true);
  });

  it("rejects a javascript: URL", () => {
    expect(isRemoteMediaUrl("javascript:alert(1)")).toBe(false);
  });

  it("rejects a data: URL", () => {
    expect(isRemoteMediaUrl("data:text/html,<script>alert(1)</script>")).toBe(false);
  });

  it("rejects a blob: URL", () => {
    expect(isRemoteMediaUrl("blob:http://example.com/uuid")).toBe(false);
  });

  it("rejects a relative path", () => {
    expect(isRemoteMediaUrl("/uploads/a.jpg")).toBe(false);
  });
});

describe("isDisplayableMediaUrl", () => {
  it("accepts http, https, and blob URLs", () => {
    expect(isDisplayableMediaUrl("http://example.com/a.jpg")).toBe(true);
    expect(isDisplayableMediaUrl("https://example.com/a.jpg")).toBe(true);
    expect(isDisplayableMediaUrl("blob:http://example.com/uuid")).toBe(true);
  });

  it("rejects a javascript: URL", () => {
    expect(isDisplayableMediaUrl("javascript:alert(1)")).toBe(false);
  });

  it("rejects a data: URL", () => {
    expect(isDisplayableMediaUrl("data:text/html,<script>alert(1)</script>")).toBe(false);
  });
});
