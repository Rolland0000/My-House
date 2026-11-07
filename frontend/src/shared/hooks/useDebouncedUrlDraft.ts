import { useEffect, useEffectEvent } from "react";
import { TEXT_FILTER_DEBOUNCE_MS, useDebouncedValue } from "./useDebouncedValue";
import { useUrlDraft } from "./useUrlDraft";

/** Draft of a text held in the URL, committed after a typing pause. An emptied field commits at once. */
export function useDebouncedUrlDraft(urlValue: string, onCommit: (value: string) => void) {
  const [draft, setDraft] = useUrlDraft(urlValue);
  const debouncedDraft = useDebouncedValue(draft, draft ? TEXT_FILTER_DEBOUNCE_MS : 0);

  // Skips a value already in the URL, which would otherwise reset the page on mount or after a resync.
  const commit = useEffectEvent((next: string) => {
    if (next.trim() !== urlValue) onCommit(next);
  });
  // Deferred past React's commit phase, where the navigation's flushSync is not allowed.
  useEffect(() => queueMicrotask(() => commit(debouncedDraft)), [debouncedDraft]);

  return [draft, setDraft] as const;
}
