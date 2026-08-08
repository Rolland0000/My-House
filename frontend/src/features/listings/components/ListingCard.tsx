import { Link } from "react-router";
import { ImageOff, MapPin } from "lucide-react";
import { Card } from "../../../shared/components";
import { formatPrice } from "../../../shared/utils/format";
import { typeLabels } from "../labels";
import type { ListingSummary } from "../api";

interface ListingCardProps {
  listing: ListingSummary;
}

function ListingCard({ listing }: ListingCardProps) {
  const location = [listing.city, listing.neighborhood].filter(Boolean).join(" · ");

  return (
    <Link
      to={`/listings/${listing.id}`}
      className="group block rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
    >
      <Card
        padding="none"
        className="overflow-hidden transition-colors group-hover:border-border-strong"
      >
        <div className="relative aspect-4/3 bg-primary-soft">
          {listing.cover_photo_url ? (
            <img
              src={listing.cover_photo_url}
              alt=""
              loading="lazy"
              className="size-full object-cover"
            />
          ) : (
            <div className="flex size-full items-center justify-center text-text-muted">
              <ImageOff className="size-8" aria-hidden="true" />
            </div>
          )}
          <span className="absolute left-2 top-2 rounded-sm bg-surface/90 px-2 py-0.5 text-sm font-semibold text-text">
            {typeLabels[listing.type]}
          </span>
          {listing.status === "unavailable" && (
            <span className="absolute right-2 top-2 rounded-sm bg-error-soft px-2 py-0.5 text-sm font-semibold text-error">
              Indisponible
            </span>
          )}
        </div>
        <div className="flex flex-col gap-1 p-4">
          <h3 className="truncate font-semibold text-text">{listing.title}</h3>
          <p className="flex items-center gap-1 text-sm text-text-muted">
            <MapPin className="size-3.5 shrink-0" aria-hidden="true" />
            <span className="truncate">{location}</span>
          </p>
          <p className="mt-1 font-semibold text-primary">{formatPrice(listing.price)}</p>
        </div>
      </Card>
    </Link>
  );
}

export { ListingCard };
export type { ListingCardProps };
