import { Link, useParams } from "react-router";
import { ArrowLeft, ArrowRight } from "lucide-react";
import { Alert, DimensionRule, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { formatFullName } from "../../../shared/utils/format";
import { isRemoteMediaUrl } from "../../../shared/utils/mediaUrl";
import { useAuth } from "../../auth";
import { useProfile } from "../../profile";
import { useListing } from "../hooks/useListings";
import { typeLabels } from "../labels";
import { ListingNotFound } from "./ListingNotFound";
import { ListingPrice } from "./ListingPrice";
import { OwnerAvatar } from "./OwnerAvatar";
import { OwnerBar } from "./OwnerBar";

const GENERIC_ERROR_MESSAGE = "Please try again in a moment.";

function ListingDetail() {
  const { id } = useParams<{ id: string }>();
  const { data, isPending, error } = useListing(id);
  const { status } = useAuth();
  const { data: profile } = useProfile({ enabled: status === "authenticated" });

  if (isPending) {
    return (
      <div className="flex justify-center py-24">
        <Spinner size="lg" label="Loading listing…" />
      </div>
    );
  }

  if (error instanceof ApiError && error.status === 404) {
    return <ListingNotFound />;
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
  const isOwner = status === "authenticated" && profile?.id === listing.owner.id;
  // Hidden until we know the viewer is not the owner, so it never flashes on their own listing.
  const showOwnerListingsLink =
    status === "anonymous" || (status === "authenticated" && !!profile && !isOwner);

  const location = [listing.city, listing.neighborhood].filter(Boolean).join(" · ");
  const ownerName = formatFullName(listing.owner.first_name, listing.owner.last_name);
  const eyebrow = [
    typeLabels[listing.type],
    listing.surface_m2 ? `${listing.surface_m2} m²` : null,
    listing.rooms ? `${listing.rooms} room${listing.rooms > 1 ? "s" : ""}` : null,
  ]
    .filter(Boolean)
    .join(" · ");

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-6 px-6 py-6">
      <Link
        to="/"
        className="flex w-fit items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900"
      >
        <ArrowLeft className="size-4" aria-hidden="true" />
        Back to listings
      </Link>

      {displayableMedia.length > 0 ? (
        <div className="flex gap-2.5 overflow-x-auto">
          {displayableMedia.map((media, index) => (
            <img
              key={media.id}
              src={media.url}
              alt={`Photo ${index + 1} of ${listing.title}`}
              className="h-64 w-96 shrink-0 object-cover"
            />
          ))}
        </div>
      ) : (
        <div className="flex h-64 items-center justify-center bg-primary-soft text-text-muted">
          No photos
        </div>
      )}

      {isOwner && (
        <OwnerBar
          listingId={listing.id}
          publishedAt={listing.published_at ?? null}
          hasPhoto={listing.media.length > 0}
        />
      )}

      <div className="flex flex-col gap-4">
        <div>
          {eyebrow && <p className="mb-1.5 text-xs font-semibold text-ink-600">{eyebrow}</p>}
          <h1 className="text-2xl font-bold text-ink-900">{listing.title}</h1>
          <p className="mt-1 text-sm font-medium text-ink-500">{location}</p>
        </div>

        <div className="flex items-baseline gap-3">
          <ListingPrice
            price={listing.price}
            className="text-2xl font-bold text-ink-900"
            unitClassName="text-sm font-medium text-ink-500"
          />
        </div>

        {listing.status === "unavailable" && (
          <Alert variant="warning">This property is currently unavailable.</Alert>
        )}

        <DimensionRule width="full" />

        <p className="whitespace-pre-line text-text">{listing.description}</p>

        <DimensionRule width="full" />

        <div>
          <h2 className="mb-2 text-lg font-bold text-ink-900">The owner</h2>
          <div className="flex items-center gap-3">
            <OwnerAvatar
              avatarUrl={listing.owner.avatar_url}
              firstName={listing.owner.first_name}
              lastName={listing.owner.last_name}
            />
            <div>
              <p className="font-semibold text-ink-900">{ownerName || "Owner"}</p>
              <p className="text-sm text-text-muted">Owner</p>
            </div>
          </div>
          {showOwnerListingsLink && (
            <Link
              to={{
                pathname: "/search",
                search: `?${new URLSearchParams({ owner_id: listing.owner.id })}`,
              }}
              className="mt-3 inline-flex items-center gap-1 text-sm font-semibold text-ink-900 underline-offset-4 hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
            >
              See other listings by {ownerName || "this owner"}
              <ArrowRight className="size-4" aria-hidden="true" />
            </Link>
          )}
          {/* role="note": a role="alert" would be announced every time the page loads. */}
          <Alert
            variant="warning"
            role="note"
            title="Before you contact this owner"
            className="mt-4"
          >
            Never send money before visiting the property. My House does not hold deposits and is
            not a party to your agreement.
          </Alert>
        </div>
      </div>
    </div>
  );
}

export { ListingDetail };
