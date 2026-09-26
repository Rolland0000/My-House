import { useMutation, useQueryClient } from "@tanstack/react-query";
import { createListing } from "../api";

export function useCreateListing() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createListing,
    retry: false,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["listings"] }),
  });
}
