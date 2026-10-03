const FIRST_PAGE = 1;

/** Reads `?page=` as a positive integer; anything else falls back to the first page. */
export function parsePageParam(value: string | null): number {
  if (value === null || !/^\d+$/.test(value)) return FIRST_PAGE;
  const page = Number(value);
  return Number.isSafeInteger(page) && page >= FIRST_PAGE ? page : FIRST_PAGE;
}
