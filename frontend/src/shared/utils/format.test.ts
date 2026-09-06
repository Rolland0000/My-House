import { describe, expect, it } from "vitest";
import { formatCountdown, formatPrice } from "./format";

describe("formatCountdown", () => {
  it("pads minutes and seconds to two digits", () => {
    expect(formatCountdown(5)).toBe("00:05");
  });

  it("formats minutes and seconds together", () => {
    expect(formatCountdown(125)).toBe("02:05");
  });

  it("formats zero as 00:00", () => {
    expect(formatCountdown(0)).toBe("00:00");
  });
});

describe("formatPrice", () => {
  it("groups thousands per the fr-FR locale", () => {
    // Intl uses a non-breaking space (U+202F) as the thousands separator.
    expect(formatPrice(1200)).toBe("1 200");
  });

  it("formats a value under one thousand without a separator", () => {
    expect(formatPrice(950)).toBe("950");
  });
});
