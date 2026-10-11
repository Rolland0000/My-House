import { Link } from "react-router";
import { Images, Pencil } from "lucide-react";
import { Alert, Badge } from "../../../shared/components";
import { VISIBILITY_BADGE } from "../labels";
import { listingVisibility, type ListingVisibility } from "../listingVisibility";

interface OwnerBarProps {
  listingId: string;
  publishedAt: string | null;
  hasPhoto: boolean;
}

const VISIBILITY_MESSAGE: Record<ListingVisibility, string | null> = {
  draft: "Only you can see this listing. Add at least one photo, then publish it.",
  published: null,
  hidden: "This listing is published but has no photos, so it's hidden from the public.",
};

function OwnerBar({ listingId, publishedAt, hasPhoto }: OwnerBarProps) {
  const visibility = listingVisibility({ publishedAt, hasPhoto });
  const badge = VISIBILITY_BADGE[visibility];
  const message = VISIBILITY_MESSAGE[visibility];

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
        </div>
      </div>
      {message && <Alert variant={visibility === "hidden" ? "warning" : "info"}>{message}</Alert>}
    </div>
  );
}

export { OwnerBar };
