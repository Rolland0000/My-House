import { useState } from "react";
import { Link, useNavigate, useParams } from "react-router";
import { ArrowLeft } from "lucide-react";
import { Alert, DimensionRule, EmptyState, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { formatPrice } from "../../../shared/utils/format";
import { isRemoteMediaUrl } from "../../../shared/utils/mediaUrl";
import { useListing } from "../hooks/useListings";
import { typeLabels } from "../labels";
import { ReportListingModal } from "./ReportListingModal";

const GENERIC_ERROR_MESSAGE = "Please try again in a moment.";

function ListingDetail() {
  const { id } = useParams<{ id: string }>();
  const { data, isPending, error } = useListing(id);
  const [isReportOpen, setIsReportOpen] = useState(false);
  const navigate = useNavigate();

  if (isPending) {
    return (
      <div className="flex justify-center py-24">
        <Spinner size="lg" label="Loading listing…" />
      </div>
    );
  }

  if (error instanceof ApiError && error.status === 404) {
    return (
      <div className="mx-auto max-w-2xl px-6 py-16">
        <EmptyState
          title="This property is no longer available"
          description="It may have been removed by the owner, or its status changed to “unavailable”."
          secondaryAction={{ label: "Back to listings", onClick: () => navigate("/") }}
        />
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

      <div className="flex flex-col gap-4">
        <div>
          {eyebrow && <p className="mb-1.5 text-xs font-semibold text-ink-600">{eyebrow}</p>}
          <h1 className="text-2xl font-bold text-ink-900">{listing.title}</h1>
          <p className="mt-1 text-sm font-medium text-ink-500">{location}</p>
        </div>

        <div className="flex items-baseline gap-3">
          <span className="text-2xl font-bold text-ink-900">
            {formatPrice(listing.price)}{" "}
            <span className="text-sm font-medium text-ink-500">FCFA / month</span>
          </span>
        </div>

        {listing.status === "unavailable" && (
          <Alert variant="warning">This property is currently unavailable.</Alert>
        )}

        <DimensionRule width="full" />

        <p className="whitespace-pre-line text-text">{listing.description}</p>

        <DimensionRule width="full" />

        <div>
          <h2 className="mb-2 text-lg font-bold text-ink-900">The owner</h2>
          <div className="flex items-center justify-between gap-3">
            <div className="flex items-center gap-3">
              <span className="flex size-11 flex-none items-center justify-center bg-ink-900 text-sm font-semibold text-white">
                {ownerName
                  .split(" ")
                  .map((part) => part[0])
                  .filter(Boolean)
                  .slice(0, 2)
                  .join("")
                  .toUpperCase() || "?"}
              </span>
              <div>
                <p className="font-semibold text-ink-900">{ownerName || "Owner"}</p>
                <p className="text-sm text-text-muted">Owner</p>
              </div>
            </div>
            <button
              type="button"
              onClick={() => setIsReportOpen(true)}
              className="text-sm font-semibold text-ink-500 hover:text-ink-900"
            >
              Report this listing
            </button>
          </div>
        </div>
      </div>

      <ReportListingModal
        isOpen={isReportOpen}
        onClose={() => setIsReportOpen(false)}
        listingTitle={listing.title}
      />
    </div>
  );
}

export { ListingDetail };
