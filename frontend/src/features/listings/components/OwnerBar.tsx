import { useId } from "react";
import { Link } from "react-router";
import { Images, Pencil } from "lucide-react";
import { Alert, Badge } from "../../../shared/components";
import { VISIBILITY_BADGE } from "../labels";
import { listingVisibility, type ListingVisibility } from "../listingVisibility";
import { PublishListingAction } from "./PublishListingAction";

interface OwnerBarProps {
  listingId: string;
  publishedAt: string | null;
  hasPhoto: boolean;
}

const VISIBILITY_MESSAGE: Record<ListingVisibility, string | null> = {
  draft: "Only you can see this listing. Publish it to make it visible to everyone.",
  published: null,
  hidden: "This listing is published but has no photos, so it's hidden from the public.",
};

const DRAFT_WITHOUT_PHOTO_MESSAGE = "Add at least one photo to publish this listing.";

function OwnerBar({ listingId, publishedAt, hasPhoto }: OwnerBarProps) {
  const visibility = listingVisibility({ publishedAt, hasPhoto });
  const badge = VISIBILITY_BADGE[visibility];
  const isDraft = visibility === "draft";
  const message =
    isDraft && !hasPhoto ? DRAFT_WITHOUT_PHOTO_MESSAGE : VISIBILITY_MESSAGE[visibility];
  const messageId = useId();

  return (
    <div className="flex flex-col gap-3 border border-border bg-surface p-4">
      <div className="flex items-center justify-between gap-3">
        <Badge tone={badge.tone}>{badge.label}</Badge>
        <div className="flex flex-wrap items-center justify-end gap-x-4 gap-y-2">
          <Link
            to={`/owner/listings/${listingId}/edit`}
            className="flex items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900"
          >
            <Pencil className="size-4" aria-hidden="true" />
            Edit listing
          </Link>
          <Link
            to={`/owner/listings/${listingId}/photos`}
            className="flex items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900"
          >
            <Images className="size-4" aria-hidden="true" />
            {hasPhoto ? "Manage photos" : "Next: add photos →"}
          </Link>
          {isDraft && (
            <PublishListingAction
              listingId={listingId}
              hasPhoto={hasPhoto}
              describedById={messageId}
            />
          )}
        </div>
      </div>
      {message && (
        <Alert id={messageId} variant={visibility === "hidden" ? "warning" : "info"}>
          {message}
        </Alert>
      )}
    </div>
  );
}

export { OwnerBar };
