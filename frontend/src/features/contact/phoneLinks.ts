/** Spaces are dropped: they are not valid in a URL. */
export function telHref(phone: string): string {
  return `tel:${phone.replace(/\s/g, "")}`;
}

/** wa.me takes the international number as digits only, so a number without `+` gets no link. */
export function whatsappHref(phone: string): string | null {
  const trimmed = phone.trim();
  if (!trimmed.startsWith("+")) return null;
  const digits = trimmed.replace(/\D/g, "");
  return digits ? `https://wa.me/${digits}` : null;
}
