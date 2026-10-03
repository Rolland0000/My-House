import { Link } from "react-router";
import { ImageOff, Pencil } from "lucide-react";
import { Badge } from "../../../shared/components";
import { formatPrice } from "../../../shared/utils/format";
import { isRemoteMediaUrl } from "../../../shared/utils/mediaUrl";
import { VISIBILITY_BADGE } from "../labels";
import { listingVisibility } from "../listingVisibility";
import type { ListingSummary } from "../api";
import { ListingStatusControl } from "./ListingStatusControl";

interface OwnerListingRowProps {
  listing: ListingSummary;
}

function OwnerListingRow({ listing }: OwnerListingRowProps) {
  const location = [listing.city, listing.neighborhood].filter(Boolean).join(" · ");
  const coverPhotoUrl =
    listing.cover_photo_url && isRemoteMediaUrl(listing.cover_photo_url)
      ? listing.cover_photo_url
      : null;
  const visibilityBadge =
    VISIBILITY_BADGE[
      listingVisibility({
        publishedAt: listing.published_at ?? null,
        hasPhoto: (listing.cover_photo_url ?? null) !== null,
      })
    ];

  return (
    <li className="relative flex flex-wrap items-center gap-4 border border-border bg-surface p-3 transition-colors has-[a:focus-visible]:ring-2 has-[a:focus-visible]:ring-focus hover:border-border-strong">
      <div className="flex h-18 w-24 flex-none items-center justify-center overflow-hidden bg-primary-soft text-text-muted">
        {coverPhotoUrl ? (
          <img src={coverPhotoUrl} alt="" loading="lazy" className="size-full object-cover" />
        ) : (
          <ImageOff className="size-6" aria-hidden="true" />
        )}
      </div>

      <div className="flex min-w-0 flex-1 flex-col gap-1.5">
        {/* The ::after overlay makes the whole row clickable without wrapping the actions in an <a>. */}
        <Link
          to={`/listings/${listing.id}`}
          className="truncate text-base font-bold text-ink-900 after:absolute after:inset-0 focus-visible:outline-none"
        >
          {listing.title}
        </Link>
        <p className="truncate text-sm font-medium text-ink-500">{location}</p>
        <p className="text-sm font-bold text-ink-900">
          {formatPrice(listing.price)}{" "}
          <span className="text-xs font-medium text-ink-500">FCFA / month</span>
        </p>
        <div className="flex flex-wrap gap-2">
          <Badge tone={visibilityBadge.tone}>{visibilityBadge.label}</Badge>
        </div>
      </div>

      <div className="relative z-10 flex w-full flex-none items-center justify-between gap-3 sm:w-auto">
        <ListingStatusControl
          listingId={listing.id}
          listingTitle={listing.title}
          status={listing.status}
        />
        <Link
          to={`/owner/listings/${listing.id}/edit`}
          aria-label={`Edit ${listing.title}`}
          className="flex items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900"
        >
          <Pencil className="size-4" aria-hidden="true" />
          Edit
        </Link>
      </div>
    </li>
  );
}

export { OwnerListingRow };
export type { OwnerListingRowProps };
