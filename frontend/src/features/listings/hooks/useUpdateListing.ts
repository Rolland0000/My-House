import { useMutation, useQueryClient } from "@tanstack/react-query";
import { updateListing, type ListingRequest } from "../api";

export function useUpdateListing(id: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: ListingRequest) => updateListing(id, body),
    retry: false,
    onSuccess: (response) => {
      queryClient.setQueryData(["listing", id], response);
      // Not returned: the caller navigates without waiting for the refetches.
      void queryClient.invalidateQueries({ queryKey: ["listings"] });
      void queryClient.invalidateQueries({ queryKey: ["owner-listings"] });
    },
  });
}
