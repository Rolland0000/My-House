import { useEffect, useState } from "react";

/**
 * Returns `value` once it has stopped changing for `delayMs`. A delay of 0
 * applies the value in the same render, with no intermediate one.
 */
export function useDebouncedValue<T>(value: T, delayMs: number): T {
  const [debouncedValue, setDebouncedValue] = useState(value);

  if (delayMs <= 0 && !Object.is(debouncedValue, value)) {
    setDebouncedValue(value);
  }

  useEffect(() => {
    if (delayMs <= 0) return;
    const timeout = window.setTimeout(() => setDebouncedValue(value), delayMs);
    return () => window.clearTimeout(timeout);
  }, [value, delayMs]);

  return debouncedValue;
}
