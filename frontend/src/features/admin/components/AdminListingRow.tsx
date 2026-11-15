import { useIsMutating } from "@tanstack/react-query";
import { Link } from "react-router";
import { Badge } from "../../../shared/components";
import { formatFullName } from "../../../shared/utils/format";
import { AVAILABILITY_BADGE, typeLabels } from "../../listings";
import { ADMIN_VISIBILITY_BADGE } from "../adminListingVisibilityBadge";
import { adminListingActionKey, type AdminListing } from "../api";
import { AdminDeleteListingAction } from "./AdminDeleteListingAction";

const CELL_CLASSES = "px-4 py-3 align-middle";

interface AdminListingRowProps {
  listing: AdminListing;
}

function AdminListingRow({ listing }: AdminListingRowProps) {
  const isActionPending = useIsMutating({ mutationKey: adminListingActionKey(listing.id) }) > 0;
  const { owner } = listing;
  const visibility = ADMIN_VISIBILITY_BADGE[listing.visibility];
  const availability = AVAILABILITY_BADGE[listing.status];

  return (
    <tr className="border-b border-border last:border-b-0">
      <td className={`${CELL_CLASSES} max-w-56 font-semibold break-words text-text`}>
        {listing.title}
      </td>
      <td className={CELL_CLASSES}>
        <Link
          to={`/admin/listings?owner_id=${owner.id}`}
          className="font-semibold text-text underline hover:text-ink-900"
        >
          {formatFullName(owner.first_name, owner.last_name) || "Unnamed owner"}
        </Link>
        <p className="text-text-muted">{owner.email}</p>
      </td>
      <td className={`${CELL_CLASSES} text-text`}>{listing.city}</td>
      <td className={`${CELL_CLASSES} text-text`}>{typeLabels[listing.type]}</td>
      <td className={CELL_CLASSES}>
        <Badge tone={availability.tone} className="whitespace-nowrap">
          {availability.label}
        </Badge>
      </td>
      <td className={CELL_CLASSES}>
        <Badge tone={visibility.tone} className="whitespace-nowrap">
          {visibility.label}
        </Badge>
      </td>
      <td className={`${CELL_CLASSES} whitespace-nowrap text-text-muted`}>
        {new Date(listing.created_at).toLocaleDateString()}
      </td>
      <td className={CELL_CLASSES}>
        <div className="flex items-center justify-end gap-4">
          {listing.visibility === "public" && (
            <Link
              to={`/listings/${listing.id}`}
              aria-label={`View ${listing.title}`}
              className="text-sm font-semibold text-ink-500 hover:text-ink-900"
            >
              View
            </Link>
          )}
          <AdminDeleteListingAction
            listingId={listing.id}
            listingTitle={listing.title}
            disabled={isActionPending}
          />
        </div>
      </td>
    </tr>
  );
}

export { AdminListingRow };
export type { AdminListingRowProps };
