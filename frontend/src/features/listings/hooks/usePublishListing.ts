import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "../../../shared/components";
import { publishListing } from "../api";
import { isPhotoRequired } from "../listingPublication";

/** Toasts live here: on success the Publish button unmounts, so `mutate()` callbacks wouldn't fire. */
export function usePublishListing(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  return useMutation({
    mutationFn: () => publishListing(id),
    retry: false,
    onSuccess: (response) => {
      queryClient.setQueryData(["listing", id], response);
      void queryClient.invalidateQueries({ queryKey: ["listings"] });
      void queryClient.invalidateQueries({ queryKey: ["owner-listings"] });
      showToast("Listing published.", { variant: "success" });
    },
    onError: (error) => {
      if (!isPhotoRequired(error)) return;
      // The last photo was removed elsewhere; reload so the bar shows the no-photo state.
      void queryClient.invalidateQueries({ queryKey: ["listing", id] });
      showToast("Add at least one photo before publishing.", { variant: "info" });
    },
  });
}
