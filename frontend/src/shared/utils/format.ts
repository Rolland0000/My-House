const priceFormatter = new Intl.NumberFormat("fr-FR");

/**
 * Formats a listing price for display.
 *
 * No currency symbol: `GET /listings` / `GET /listings/:id` don't return one
 * (see `listings.price` in `TECHNICAL_SPEC_MVP.md §4.3`) — display currency
 * for the France vs. Africa markets is an open call tied to the monetization
 * blocker noted in `CLAUDE.md`, not something to invent here.
 */
export function formatPrice(price: number): string {
  return priceFormatter.format(price);
}

export function formatCountdown(totalSeconds: number): string {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}
