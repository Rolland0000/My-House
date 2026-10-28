const priceFormatter = new Intl.NumberFormat("fr-FR");

export const PRICE_CURRENCY = "XAF";

/** Amount only, grouped per the fr-FR locale (e.g. "150 000"). */
export function formatPrice(price: number): string {
  return priceFormatter.format(price);
}

/** Every listing price is a monthly rent in XAF; split so views can style each part. */
export function formatMonthlyRent(price: number): { amount: string; unit: string } {
  return { amount: formatPrice(price), unit: `${PRICE_CURRENCY} / month` };
}

export function formatCountdown(totalSeconds: number): string {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}

/** First and last name joined by a space, skipping blank parts; "" when both are blank. */
export function formatFullName(firstName?: string | null, lastName?: string | null): string {
  return [firstName, lastName]
    .map((part) => part?.trim())
    .filter(Boolean)
    .join(" ");
}

/** Initials for the ink-900 avatar square used throughout the header/profile/detail. */
export function formatInitials(firstName?: string | null, lastName?: string | null): string {
  return [firstName, lastName]
    .map((part) => part?.trim()?.[0])
    .filter(Boolean)
    .join("")
    .toUpperCase();
}
