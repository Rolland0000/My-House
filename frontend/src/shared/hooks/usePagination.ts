import { useState } from "react";

/**
 * Local page-number state that resets to 1 whenever `resetKey` changes —
 * prevents being stuck on a page number that no longer exists in the result
 * set after a filter change (e.g. going from page 3 to a filtered set with
 * only 1 page).
 *
 * Resets during render (React's documented pattern for "adjusting state when
 * a prop changes") rather than in a `useEffect`, which would cause an extra
 * render pass.
 */
export function usePagination(resetKey?: unknown) {
  const [page, setPage] = useState(1);
  const [prevResetKey, setPrevResetKey] = useState(resetKey);

  if (resetKey !== prevResetKey) {
    setPrevResetKey(resetKey);
    setPage(1);
  }

  return { page, setPage };
}
