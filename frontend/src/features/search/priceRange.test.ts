import { describe, expect, it } from "vitest";
import { formatPrice } from "../../shared/utils/format";
import { PRICE_MAX } from "../listings";
import {
  clampHandle,
  formatPriceLabel,
  parsePriceInput,
  PRICE_SLIDER,
  resolveCrossing,
  toHandlePosition,
  toPriceBounds,
} from "./priceRange";

describe("toHandlePosition", () => {
  it("snaps to the nearest step", () => {
    expect(toHandlePosition(0)).toBe(0);
    expect(toHandlePosition(7_499)).toBe(5_000);
    expect(toHandlePosition(7_500)).toBe(10_000);
  });

  it("stops at the top of the slider", () => {
    expect(toHandlePosition(PRICE_SLIDER.max)).toBe(PRICE_SLIDER.max);
    expect(toHandlePosition(2_500_000)).toBe(PRICE_SLIDER.max);
  });
});

describe("clampHandle", () => {
  it("keeps a position between the given limits", () => {
    expect(clampHandle(-PRICE_SLIDER.pageStep, 0, PRICE_SLIDER.max)).toBe(0);
    expect(clampHandle(300_000, 0, 250_000)).toBe(250_000);
    expect(clampHandle(200_000, 250_000, PRICE_SLIDER.max)).toBe(250_000);
    expect(clampHandle(400_000, 250_000, PRICE_SLIDER.max)).toBe(400_000);
  });
});

describe("parsePriceInput", () => {
  it("falls back to the slider end of the bound when empty", () => {
    expect(parsePriceInput("", "min")).toBe(PRICE_SLIDER.min);
    expect(parsePriceInput("  ", "max")).toBe(PRICE_SLIDER.max);
  });

  it("falls back when unreadable or negative", () => {
    expect(parsePriceInput("abc", "min")).toBe(PRICE_SLIDER.min);
    expect(parsePriceInput("-5", "max")).toBe(PRICE_SLIDER.max);
  });

  it("keeps any whole amount and rounds a decimal", () => {
    expect(parsePriceInput("7499", "min")).toBe(7_499);
    expect(parsePriceInput("1500000", "max")).toBe(1_500_000);
    expect(parsePriceInput("12.5", "min")).toBe(13);
  });

  it("caps an amount at the highest listing price", () => {
    expect(parsePriceInput("99999999999999", "max")).toBe(PRICE_MAX);
  });
});

describe("resolveCrossing", () => {
  it("sets an edited minimum above the maximum to the maximum", () => {
    expect(resolveCrossing(300_000, 200_000, "min")).toEqual({ min: 200_000, max: 200_000 });
  });

  it("sets an edited maximum below the minimum to the minimum", () => {
    expect(resolveCrossing(300_000, 200_000, "max")).toEqual({ min: 300_000, max: 300_000 });
  });

  it("keeps ordered or equal bounds", () => {
    expect(resolveCrossing(100_000, 200_000, "min")).toEqual({ min: 100_000, max: 200_000 });
    expect(resolveCrossing(200_000, 200_000, "max")).toEqual({ min: 200_000, max: 200_000 });
  });

  it("never crosses a maximum at the top of the slider", () => {
    expect(resolveCrossing(2_000_000, PRICE_SLIDER.max, "min")).toEqual({
      min: 2_000_000,
      max: PRICE_SLIDER.max,
    });
  });
});

describe("toPriceBounds", () => {
  it("drops the slider ends", () => {
    expect(toPriceBounds(PRICE_SLIDER.min, PRICE_SLIDER.max)).toEqual({
      priceMin: null,
      priceMax: null,
    });
  });

  it("keeps any other amount, including one above the slider", () => {
    expect(toPriceBounds(50_000, 1_500_000)).toEqual({ priceMin: 50_000, priceMax: 1_500_000 });
    expect(toPriceBounds(0, 0)).toEqual({ priceMin: null, priceMax: 0 });
  });
});

describe("formatPriceLabel", () => {
  it("formats the amount in XAF", () => {
    expect(formatPriceLabel(50_000)).toBe(`${formatPrice(50_000)} XAF`);
  });

  it("marks an open-ended amount", () => {
    expect(formatPriceLabel(PRICE_SLIDER.max, true)).toBe(`${formatPrice(PRICE_SLIDER.max)}+ XAF`);
  });
});
