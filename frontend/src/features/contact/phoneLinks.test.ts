import { describe, expect, it } from "vitest";
import { telHref, whatsappHref } from "./phoneLinks";

describe("telHref", () => {
  it("drops the spaces", () => {
    expect(telHref("+237 699 000 111")).toBe("tel:+237699000111");
  });

  it("keeps a local number as is", () => {
    expect(telHref("691676726")).toBe("tel:691676726");
  });
});

describe("whatsappHref", () => {
  it("keeps only the digits of an international number", () => {
    expect(whatsappHref("+237 699 000 111")).toBe("https://wa.me/237699000111");
  });

  it("ignores surrounding spaces", () => {
    expect(whatsappHref("  +2250700000000 ")).toBe("https://wa.me/2250700000000");
  });

  it("returns null for a number without +", () => {
    expect(whatsappHref("691676726")).toBeNull();
  });

  it("returns null for a + with no digits", () => {
    expect(whatsappHref("+")).toBeNull();
  });
});
