import { formatFullName } from "./format";

interface OwnedRow {
  owner: { id: string; first_name?: string | null; last_name?: string | null };
}

/** Chip label for the owner filter, named after a result that belongs to that owner. */
export function ownerFilterLabel(ownerId: string, listings: readonly OwnedRow[]): string {
  // Results kept on screen while loading may belong to a previously filtered owner.
  const owner = listings.find((listing) => listing.owner.id === ownerId)?.owner;
  const name = owner ? formatFullName(owner.first_name, owner.last_name) : "";
  return `Listings by ${name || "this owner"}`;
}
