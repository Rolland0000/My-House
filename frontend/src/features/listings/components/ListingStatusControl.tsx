import { Check } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import type { ListingStatus } from "../api";
import { useUpdateListingStatus } from "../hooks/useUpdateListingStatus";
import { AVAILABILITY_BADGE } from "../labels";

const STATUSES: ListingStatus[] = ["available", "unavailable"];

const CHECKED_CLASSES: Record<ListingStatus, string> = {
  available: "has-checked:bg-success-soft has-checked:text-success",
  unavailable: "has-checked:bg-error-soft has-checked:text-error-text",
};

interface ListingStatusControlProps {
  listingId: string;
  listingTitle: string;
  status: ListingStatus;
}

function ListingStatusControl({ listingId, listingTitle, status }: ListingStatusControlProps) {
  const { mutate, isPending } = useUpdateListingStatus(listingId);

  return (
    // aria-disabled rather than `disabled`: disabling the focused radio sends keyboard focus to <body>.
    <fieldset
      aria-disabled={isPending}
      className="flex rounded-sm border border-border text-xs font-semibold aria-disabled:opacity-60"
    >
      <legend className="sr-only">Availability of {listingTitle}</legend>
      {STATUSES.map((value) => (
        <label
          key={value}
          className={cn(
            "flex cursor-pointer items-center gap-1 px-2.5 py-1.5 text-ink-500 first:rounded-l-sm last:rounded-r-sm has-focus-visible:ring-2 has-focus-visible:ring-focus",
            isPending && "cursor-not-allowed",
            CHECKED_CLASSES[value]
          )}
        >
          {/* sr-only keeps the input focusable and announced; `hidden` would drop it from the tab order. */}
          <input
            type="radio"
            name={`status-${listingId}`}
            value={value}
            checked={status === value}
            aria-disabled={isPending}
            onChange={() => {
              if (!isPending) mutate({ status: value, previousStatus: status });
            }}
            className="sr-only"
          />
          {/* Shows the selection without relying on color. */}
          {status === value && <Check className="size-3.5" aria-hidden="true" />}
          {AVAILABILITY_BADGE[value].label}
        </label>
      ))}
    </fieldset>
  );
}

export { ListingStatusControl };
export type { ListingStatusControlProps };
