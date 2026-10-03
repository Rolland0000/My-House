import { useMutation, useQueryClient } from "@tanstack/react-query";
import { ApiError } from "../../../shared/api/client";
import { useToast } from "../../../shared/components";
import { updateListingStatus, type ListListingsResult, type ListingStatus } from "../api";
import { withListingStatus, withoutListing } from "../ownerListingsCache";

const OWNER_LISTINGS_KEY = ["owner-listings"];
export const LISTING_STATUS_MUTATION_KEY = ["listing-status"];

interface StatusChange {
  status: ListingStatus;
  previousStatus: ListingStatus;
}

export function useUpdateListingStatus(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  const patchOwnerListings = (
    update: (result: ListListingsResult | undefined) => ListListingsResult | undefined
  ) => queryClient.setQueriesData<ListListingsResult>({ queryKey: OWNER_LISTINGS_KEY }, update);

  return useMutation({
    // The id lets a row see its own pending change; filters on the prefix still match every row.
    mutationKey: [...LISTING_STATUS_MUTATION_KEY, id],
    mutationFn: ({ status }: StatusChange) => updateListingStatus(id, status),
    retry: false,
    onMutate: async ({ status }) => {
      await queryClient.cancelQueries({ queryKey: OWNER_LISTINGS_KEY });
      patchOwnerListings((result) => withListingStatus(result, id, status));
    },
    onError: (error, { previousStatus }) => {
      if (error instanceof ApiError && error.status === 404) {
        patchOwnerListings((result) => withoutListing(result, id));
        showToast("This listing no longer exists.", { variant: "info" });
        return;
      }
      // Rolls back this row only, so a concurrent change on another row keeps its state.
      patchOwnerListings((result) => withListingStatus(result, id, previousStatus));
      showToast("Couldn't update the availability. Please try again.", { variant: "error" });
    },
    onSettled: () => {
      void queryClient.invalidateQueries({ queryKey: ["listings"] });
      void queryClient.invalidateQueries({ queryKey: ["listing", id] });
      // Still counted as pending here. Only the last pending change refetches the list,
      // so the refetch can't overwrite another row's optimistic state.
      if (queryClient.isMutating({ mutationKey: LISTING_STATUS_MUTATION_KEY }) === 1) {
        void queryClient.invalidateQueries({ queryKey: OWNER_LISTINGS_KEY });
      }
    },
  });
}
