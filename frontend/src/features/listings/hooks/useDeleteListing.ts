import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "../../../shared/components";
import { deleteListing, type ListListingsResult } from "../api";
import { isAlreadyDeleted } from "../listingDeletion";
import { withoutListing } from "../ownerListingsCache";
import { LISTING_STATUS_MUTATION_KEY } from "./useUpdateListingStatus";

const OWNER_LISTINGS_KEY = ["owner-listings"];

/** Success is handled in the hook because the row unmounts on success, and `mutate()` callbacks don't fire after that. */
export function useDeleteListing(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  return useMutation({
    mutationFn: async () => {
      try {
        await deleteListing(id);
      } catch (error) {
        if (!isAlreadyDeleted(error)) throw error;
      }
    },
    retry: false,
    onSuccess: () => {
      queryClient.setQueriesData<ListListingsResult>({ queryKey: OWNER_LISTINGS_KEY }, (result) =>
        withoutListing(result, id)
      );
      queryClient.removeQueries({ queryKey: ["listing", id] });
      void queryClient.invalidateQueries({ queryKey: ["listings"] });
      // A pending status change refetches the list when it settles; refetching now
      // would overwrite its optimistic state.
      if (queryClient.isMutating({ mutationKey: LISTING_STATUS_MUTATION_KEY }) === 0) {
        void queryClient.invalidateQueries({ queryKey: OWNER_LISTINGS_KEY });
      }
      showToast("Listing deleted.", { variant: "success" });
    },
  });
}
