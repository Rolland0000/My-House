import { useEffect, useEffectEvent, useId, useRef, type KeyboardEvent } from "react";
import { Input } from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";
import { PRICE_MAX } from "../../listings";
import { useUrlDraft } from "../hooks/useUrlDraft";
import {
  clampHandle,
  formatPriceLabel,
  parsePriceInput,
  PRICE_SLIDER,
  resolveCrossing,
  toHandlePosition,
  toPriceBounds,
  type PriceBound,
  type PriceBounds,
} from "../priceRange";

interface PriceRangeFilterProps extends PriceBounds {
  onCommit: (bounds: PriceBounds) => void;
}

const SLIDER_SPAN = PRICE_SLIDER.max - PRICE_SLIDER.min;

// Both ranges share one track: only their thumbs take pointer input, so either can be grabbed.
const RANGE_CLASSES = cn(
  "pointer-events-none absolute inset-0 h-full w-full appearance-none bg-transparent focus-visible:outline-none",
  "[&::-webkit-slider-thumb]:pointer-events-auto [&::-webkit-slider-thumb]:size-6 [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:border-2 [&::-webkit-slider-thumb]:border-ink-900 [&::-webkit-slider-thumb]:bg-surface",
  "focus-visible:[&::-webkit-slider-thumb]:outline-2 focus-visible:[&::-webkit-slider-thumb]:outline-offset-2 focus-visible:[&::-webkit-slider-thumb]:outline-focus",
  "[&::-moz-range-track]:bg-transparent [&::-moz-range-thumb]:pointer-events-auto [&::-moz-range-thumb]:box-border [&::-moz-range-thumb]:size-6 [&::-moz-range-thumb]:cursor-pointer [&::-moz-range-thumb]:rounded-none [&::-moz-range-thumb]:border-2 [&::-moz-range-thumb]:border-ink-900 [&::-moz-range-thumb]:bg-surface",
  "focus-visible:[&::-moz-range-thumb]:outline-2 focus-visible:[&::-moz-range-thumb]:outline-offset-2 focus-visible:[&::-moz-range-thumb]:outline-focus"
);

// Spin buttons are hidden: they crowd the amount in a narrow sidebar.
const AMOUNT_FIELD_CLASSES =
  "[appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none";

function toPercent(position: number): number {
  return ((position - PRICE_SLIDER.min) / SLIDER_SPAN) * 100;
}

function PriceRangeFilter({ priceMin, priceMax, onCommit }: PriceRangeFilterProps) {
  const minFieldId = useId();
  const maxFieldId = useId();
  const minRangeRef = useRef<HTMLInputElement>(null);
  const maxRangeRef = useRef<HTMLInputElement>(null);
  const [minDraft, setMinDraft] = useUrlDraft(String(priceMin ?? PRICE_SLIDER.min));
  const [maxDraft, setMaxDraft] = useUrlDraft(String(priceMax ?? PRICE_SLIDER.max));

  const minAmount = parsePriceInput(minDraft, "min");
  const maxAmount = parsePriceInput(maxDraft, "max");
  const maxPosition = toHandlePosition(maxAmount);
  // A typed minimum may exceed the maximum until the field is left; its handle stops at the other one.
  const minPosition = Math.min(toHandlePosition(minAmount), maxPosition);
  const isMaxOpenEnded = maxAmount === PRICE_SLIDER.max;

  function commit(min: number, max: number, edited: PriceBound) {
    const resolved = resolveCrossing(min, max, edited);
    setMinDraft(String(resolved.min));
    setMaxDraft(String(resolved.max));
    onCommit(toPriceBounds(resolved.min, resolved.max));
  }

  function clampToOtherHandle(bound: PriceBound, position: number): number {
    return bound === "min"
      ? clampHandle(position, PRICE_SLIDER.min, maxPosition)
      : clampHandle(position, minPosition, PRICE_SLIDER.max);
  }

  function moveHandle(bound: PriceBound, position: number) {
    const draft = String(clampToOtherHandle(bound, position));
    if (bound === "min") setMinDraft(draft);
    else setMaxDraft(draft);
  }

  function commitHandle(bound: PriceBound, position: number) {
    const clamped = clampToOtherHandle(bound, position);
    if (bound === "min") commit(clamped, maxAmount, "min");
    else commit(minAmount, clamped, "max");
  }

  // Browsers vary on the Page Up / Page Down step, so it is set here.
  function handleRangeKeyDown(bound: PriceBound, event: KeyboardEvent<HTMLInputElement>) {
    const direction = event.key === "PageUp" ? 1 : event.key === "PageDown" ? -1 : 0;
    if (direction === 0) return;
    event.preventDefault();
    const current = bound === "min" ? minPosition : maxPosition;
    commitHandle(bound, current + direction * PRICE_SLIDER.pageStep);
  }

  function handleFieldKeyDown(bound: PriceBound, event: KeyboardEvent<HTMLInputElement>) {
    if (event.key === "Enter") commit(minAmount, maxAmount, bound);
  }

  // React's onChange fires on every move; the native change event fires once the handle is released.
  const onHandleRelease = useEffectEvent(commitHandle);
  useEffect(() => {
    const minRange = minRangeRef.current;
    const maxRange = maxRangeRef.current;
    if (!minRange || !maxRange) return;
    const releaseMin = () => onHandleRelease("min", minRange.valueAsNumber);
    const releaseMax = () => onHandleRelease("max", maxRange.valueAsNumber);
    minRange.addEventListener("change", releaseMin);
    maxRange.addEventListener("change", releaseMax);
    return () => {
      minRange.removeEventListener("change", releaseMin);
      maxRange.removeEventListener("change", releaseMax);
    };
  }, []);

  const minLabel = formatPriceLabel(minAmount);
  const maxLabel = formatPriceLabel(maxAmount, isMaxOpenEnded);

  return (
    <fieldset className="flex flex-col gap-3">
      <legend className="mb-3 text-sm font-semibold text-text">Monthly rent (XAF)</legend>
      <p className="text-sm text-ink-500">
        {minLabel} – {maxLabel}
      </p>

      <div className="relative h-7">
        <div className="absolute inset-x-3 top-1/2 h-1 -translate-y-1/2 bg-border-strong">
          <div
            className="absolute inset-y-0 bg-ink-900"
            style={{
              left: `${toPercent(minPosition)}%`,
              right: `${100 - toPercent(maxPosition)}%`,
            }}
          />
        </div>
        <input
          ref={minRangeRef}
          type="range"
          aria-label="Minimum rent"
          aria-valuetext={minLabel}
          min={PRICE_SLIDER.min}
          max={PRICE_SLIDER.max}
          step={PRICE_SLIDER.step}
          value={minPosition}
          onChange={(event) => moveHandle("min", event.target.valueAsNumber)}
          onKeyDown={(event) => handleRangeKeyDown("min", event)}
          // With both handles at the top, only the minimum can move, so it goes on top.
          className={cn(RANGE_CLASSES, minPosition === PRICE_SLIDER.max && "z-10")}
        />
        <input
          ref={maxRangeRef}
          type="range"
          aria-label="Maximum rent"
          aria-valuetext={maxLabel}
          min={PRICE_SLIDER.min}
          max={PRICE_SLIDER.max}
          step={PRICE_SLIDER.step}
          value={maxPosition}
          onChange={(event) => moveHandle("max", event.target.valueAsNumber)}
          onKeyDown={(event) => handleRangeKeyDown("max", event)}
          className={RANGE_CLASSES}
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="flex flex-col gap-1.5">
          <label htmlFor={minFieldId} className="text-sm font-semibold text-text">
            Min
          </label>
          <Input
            id={minFieldId}
            type="number"
            inputMode="numeric"
            min={PRICE_SLIDER.min}
            max={PRICE_MAX}
            className={AMOUNT_FIELD_CLASSES}
            value={minDraft}
            onChange={(event) => setMinDraft(event.target.value)}
            onBlur={() => commit(minAmount, maxAmount, "min")}
            onKeyDown={(event) => handleFieldKeyDown("min", event)}
          />
        </div>
        <div className="flex flex-col gap-1.5">
          <label htmlFor={maxFieldId} className="text-sm font-semibold text-text">
            Max
          </label>
          <Input
            id={maxFieldId}
            type="number"
            inputMode="numeric"
            min={PRICE_SLIDER.min}
            max={PRICE_MAX}
            className={AMOUNT_FIELD_CLASSES}
            value={maxDraft}
            onChange={(event) => setMaxDraft(event.target.value)}
            onBlur={() => commit(minAmount, maxAmount, "max")}
            onKeyDown={(event) => handleFieldKeyDown("max", event)}
          />
        </div>
      </div>
    </fieldset>
  );
}

export { PriceRangeFilter };
