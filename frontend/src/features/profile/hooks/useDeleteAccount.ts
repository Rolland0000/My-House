import { useMutation, useQueryClient } from "@tanstack/react-query";
import { deleteAccount } from "../api";

/** Clears the entire query cache on success: every cached record is scoped
 *  to the now-deleted account. */
export function useDeleteAccount() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteAccount,
    retry: false,
    onSuccess: () => queryClient.clear(),
  });
}
