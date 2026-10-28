import { formatFullName } from "../../shared/utils/format";
import type { ListingSummary } from "../listings";

/** Chip label for the owner filter, named after a result that belongs to that owner. */
export function ownerFilterLabel(
  ownerId: string,
  listings: readonly Pick<ListingSummary, "owner">[]
): string {
  // Results kept on screen while loading may belong to a previously filtered owner.
  const owner = listings.find((listing) => listing.owner.id === ownerId)?.owner;
  const name = owner ? formatFullName(owner.first_name, owner.last_name) : "";
  return `Listings by ${name || "this owner"}`;
}
