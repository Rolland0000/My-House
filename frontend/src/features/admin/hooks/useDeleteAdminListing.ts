import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "../../../shared/components";
import { isAlreadyDeleted } from "../../listings";
import { adminListingActionKey, deleteAdminListing } from "../api";
import { LISTING_GONE_MESSAGE, refreshModeratedListing } from "./useAdminListings";

/** Success is handled in the hook because the row unmounts on success, and `mutate()` callbacks don't fire after that. */
export function useDeleteAdminListing(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  return useMutation({
    mutationKey: adminListingActionKey(id),
    mutationFn: async (): Promise<"deleted" | "gone"> => {
      try {
        await deleteAdminListing(id);
        return "deleted";
      } catch (error) {
        if (isAlreadyDeleted(error)) return "gone";
        throw error;
      }
    },
    retry: false,
    onSuccess: (outcome) => {
      if (outcome === "deleted") {
        showToast("Listing deleted.", { variant: "success" });
      } else {
        showToast(LISTING_GONE_MESSAGE, { variant: "info" });
      }
      return refreshModeratedListing(queryClient, id);
    },
  });
}
