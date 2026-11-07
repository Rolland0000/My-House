import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "../../../shared/components";
import { isAlreadyDeleted, type ListingStatus } from "../../listings";
import { adminListingActionKey, updateAdminListingStatus } from "../api";
import { LISTING_GONE_MESSAGE, refreshModeratedListing } from "./useAdminListings";

export function useUpdateAdminListingStatus(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  return useMutation({
    mutationKey: adminListingActionKey(id),
    mutationFn: (status: ListingStatus) => updateAdminListingStatus(id, status),
    retry: false,
    // Returning the promise keeps the row locked until the refetch shows the new status.
    onSuccess: () => refreshModeratedListing(queryClient, id),
    onError: async (error) => {
      if (isAlreadyDeleted(error)) {
        showToast(LISTING_GONE_MESSAGE, { variant: "info" });
        await refreshModeratedListing(queryClient, id);
        return;
      }
      showToast("Couldn't update the status. Please try again.", { variant: "error" });
    },
  });
}
