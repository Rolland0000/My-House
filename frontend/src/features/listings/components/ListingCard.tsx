import { Link } from "react-router";
import { ImageOff } from "lucide-react";
import { Card } from "../../../shared/components";
import { formatPrice } from "../../../shared/utils/format";
import { isRemoteMediaUrl } from "../../../shared/utils/mediaUrl";
import { typeLabels } from "../labels";
import type { ListingSummary } from "../api";

interface ListingCardProps {
  listing: ListingSummary;
}

function ListingCard({ listing }: ListingCardProps) {
  const location = [listing.city, listing.neighborhood].filter(Boolean).join(" · ");

  const coverPhotoUrl =
    listing.cover_photo_url && isRemoteMediaUrl(listing.cover_photo_url)
      ? listing.cover_photo_url
      : null;

  return (
    <Link
      to={`/listings/${listing.id}`}
      className="group block focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
    >
      <Card radius="none" padding="none" className="transition-colors group-hover:border-border-strong">
        <div className="relative aspect-4/3 overflow-hidden bg-primary-soft">
          {coverPhotoUrl ? (
            <img
              src={coverPhotoUrl}
              alt={listing.title}
              loading="lazy"
              className="size-full object-cover"
            />
          ) : (
            <div className="flex size-full items-center justify-center text-text-muted">
              <ImageOff className="size-8" aria-hidden="true" />
            </div>
          )}
          <span className="absolute top-0 left-0 border-r border-b border-border bg-surface px-2.5 py-1.5 text-xs font-semibold text-ink-900">
            {typeLabels[listing.type]}
          </span>
          {listing.status === "unavailable" && (
            <span className="absolute top-2 right-2 bg-error-soft px-2 py-0.5 text-xs font-semibold text-error-text">
              Unavailable
            </span>
          )}
          <span className="absolute bottom-0 left-3 border border-b-0 border-brass-600 bg-surface px-3 py-2 text-[17px] leading-none font-bold text-ink-900">
            {formatPrice(listing.price)} <span className="text-[11.5px] font-medium text-ink-500">FCFA / month</span>
          </span>
        </div>
        <div className="flex flex-col gap-1.5 p-4">
          <h3 className="truncate text-base font-bold text-ink-900">{listing.title}</h3>
          <p className="truncate text-sm font-medium text-ink-500">{location}</p>
        </div>
      </Card>
    </Link>
  );
}

export { ListingCard };
export type { ListingCardProps };
