import { describe, expect, it } from "vitest";
import { formatCountdown, formatFullName, formatMonthlyRent, formatPrice } from "./format";

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

describe("formatMonthlyRent", () => {
  it("groups thousands and labels the amount as a monthly rent in XAF", () => {
    expect(formatMonthlyRent(150000)).toEqual({ amount: "150 000", unit: "XAF / month" });
  });

  it("renders a whole amount with no decimals", () => {
    expect(formatMonthlyRent(950).amount).toBe("950");
  });
});

describe("formatFullName", () => {
  it("joins first and last name", () => {
    expect(formatFullName("Fatou", "Ndiaye")).toBe("Fatou Ndiaye");
  });

  it("keeps the part that is present", () => {
    expect(formatFullName("Fatou", null)).toBe("Fatou");
    expect(formatFullName(undefined, "Ndiaye")).toBe("Ndiaye");
  });

  it("trims parts and returns an empty string when both are blank", () => {
    expect(formatFullName("  Fatou ", " ")).toBe("Fatou");
    expect(formatFullName(" ", null)).toBe("");
  });
});
