import type { ListingStatus } from "../api";
import { useUpdateListingStatus } from "../hooks/useUpdateListingStatus";
import { ListingStatusSwitch } from "./ListingStatusSwitch";

interface ListingStatusControlProps {
  listingId: string;
  listingTitle: string;
  status: ListingStatus;
}

function ListingStatusControl({ listingId, listingTitle, status }: ListingStatusControlProps) {
  const { mutate, isPending } = useUpdateListingStatus(listingId);

  return (
    <ListingStatusSwitch
      listingId={listingId}
      listingTitle={listingTitle}
      status={status}
      isPending={isPending}
      onChange={(nextStatus) => mutate({ status: nextStatus, previousStatus: status })}
    />
  );
}

export { ListingStatusControl };
export type { ListingStatusControlProps };
