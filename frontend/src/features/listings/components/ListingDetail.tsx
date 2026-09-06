import { Link, useParams } from "react-router";
import { ArrowLeft, MapPin } from "lucide-react";
import { Alert, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { formatPrice } from "../../../shared/utils/format";
import { isRemoteMediaUrl } from "../../../shared/utils/mediaUrl";
import { useListing } from "../hooks/useListings";
import { typeLabels } from "../labels";

const GENERIC_ERROR_MESSAGE = "Please try again in a moment.";

function ListingDetail() {
  const { id } = useParams<{ id: string }>();
  const { data, isPending, error } = useListing(id);

  if (isPending) {
    return (
      <div className="flex justify-center py-24">
        <Spinner size="lg" label="Loading listing…" />
      </div>
    );
  }

  if (error instanceof ApiError && error.status === 404) {
    return (
      <div className="mx-auto flex max-w-2xl flex-col items-center gap-3 py-24 text-center">
        <h1 className="text-lg font-bold text-text">This property is no longer available</h1>
        <p className="text-text-muted">
          It may have been removed by the owner, or its status changed to “unavailable”.
        </p>
        <Link to="/" className="font-semibold text-primary hover:underline">
          ← Back to feed
        </Link>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        <Alert variant="error" title="Unable to load this listing">
          {error instanceof ApiError ? GENERIC_ERROR_MESSAGE : GENERIC_ERROR_MESSAGE}
        </Alert>
      </div>
    );
  }

  const listing = data.data;
  const displayableMedia = listing.media.filter((media) => isRemoteMediaUrl(media.url));

  const location = [listing.city, listing.neighborhood].filter(Boolean).join(" · ");
  const ownerName = [listing.owner.first_name, listing.owner.last_name].filter(Boolean).join(" ");
  const stats = [
    listing.surface_m2 ? `${listing.surface_m2} m²` : null,
    listing.rooms ? `${listing.rooms} room${listing.rooms > 1 ? "s" : ""}` : null,
  ]
    .filter(Boolean)
    .join(" · ");

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-6 p-6">
      <Link
        to="/"
        className="flex w-fit items-center gap-1 text-sm font-semibold text-text-muted hover:text-text"
      >
        <ArrowLeft className="size-4" aria-hidden="true" />
        Back to feed
      </Link>

      {displayableMedia.length > 0 ? (
        <div className="flex gap-3 overflow-x-auto rounded-md">
          {displayableMedia.map((media, index) => (
            <img
              key={media.id}
              src={media.url}
              alt={`Photo ${index + 1} of ${listing.title}`}
              className="h-64 w-96 shrink-0 rounded-md object-cover"
            />
          ))}
        </div>
      ) : (
        <div className="flex h-64 items-center justify-center rounded-md bg-primary-soft text-text-muted">
          No photos
        </div>
      )}

      <div className="flex flex-col gap-4">
        <div className="flex flex-wrap items-start justify-between gap-2">
          <div>
            <h1 className="text-lg font-bold text-text">{listing.title}</h1>
            <p className="mt-1 flex items-center gap-1 text-text-muted">
              <MapPin className="size-4 shrink-0" aria-hidden="true" />
              {location}
            </p>
          </div>
          <span className="rounded-sm bg-primary-soft px-2 py-1 text-sm font-semibold text-text">
            {typeLabels[listing.type]}
          </span>
        </div>

        <div className="flex items-baseline gap-3">
          <span className="text-lg font-bold text-text">{formatPrice(listing.price)}</span>
          {stats && <span className="text-sm text-text-muted">{stats}</span>}
        </div>

        {listing.status === "unavailable" && (
          <Alert variant="warning">This property is currently unavailable.</Alert>
        )}

        <p className="whitespace-pre-line text-text">{listing.description}</p>

        <div className="flex items-center gap-3 border-t border-border pt-4">
          <div>
            <p className="font-semibold text-text">{ownerName || "Owner"}</p>
            <p className="text-sm text-text-muted">Owner</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export { ListingDetail };
