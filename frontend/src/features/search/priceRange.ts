import { formatPrice, PRICE_CURRENCY } from "../../shared/utils/format";
import { PRICE_MAX } from "../listings";

/** Slider bounds and steps in whole XAF, kept together so they can be revised against real prices. */
export const PRICE_SLIDER = { min: 0, max: 1_000_000, step: 5_000, pageStep: 50_000 } as const;

export type PriceBound = "min" | "max";

export interface PriceBounds {
  priceMin: number | null;
  priceMax: number | null;
}

/** Nearest slider step to an amount, clamped to the slider range. */
export function toHandlePosition(amount: number): number {
  const snapped = Math.round(amount / PRICE_SLIDER.step) * PRICE_SLIDER.step;
  return clampHandle(snapped, PRICE_SLIDER.min, PRICE_SLIDER.max);
}

export function clampHandle(position: number, lowest: number, highest: number): number {
  return Math.min(highest, Math.max(lowest, position));
}

/** Reads a typed amount; empty or unreadable falls back to the slider end of that bound. */
export function parsePriceInput(raw: string, bound: PriceBound): number {
  const fallback = bound === "min" ? PRICE_SLIDER.min : PRICE_SLIDER.max;
  const trimmed = raw.trim();
  if (!trimmed) return fallback;
  const amount = Number(trimmed);
  if (!Number.isFinite(amount) || amount < 0) return fallback;
  return Math.min(Math.round(amount), PRICE_MAX);
}

/** Sets a crossed bound to the other one; a maximum at the slider top has no bound to cross. */
export function resolveCrossing(
  min: number,
  max: number,
  edited: PriceBound
): { min: number; max: number } {
  if (max === PRICE_SLIDER.max || min <= max) return { min, max };
  return edited === "min" ? { min: max, max } : { min, max: min };
}

/** The slider ends mean "no bound": the API gets neither 0 nor the top value. */
export function toPriceBounds(min: number, max: number): PriceBounds {
  return {
    priceMin: min === PRICE_SLIDER.min ? null : min,
    priceMax: max === PRICE_SLIDER.max ? null : max,
  };
}

export function formatPriceLabel(amount: number, isOpenEnded = false): string {
  return `${formatPrice(amount)}${isOpenEnded ? "+" : ""} ${PRICE_CURRENCY}`;
}
