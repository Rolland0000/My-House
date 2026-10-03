import { useMutation, useQueryClient } from "@tanstack/react-query";
import { createListing } from "../api";

export function useCreateListing() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createListing,
    retry: false,
    onSuccess: () => {
      // Not returned: the caller navigates without waiting for the feed refetch.
      void queryClient.invalidateQueries({ queryKey: ["listings"] });
      void queryClient.invalidateQueries({ queryKey: ["owner-listings"] });
    },
  });
}
