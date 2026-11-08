import { useMutation, useQueryClient } from "@tanstack/react-query";
import { isNotFound } from "../../../shared/api/client";
import { useToast } from "../../../shared/components";
import {
  ADMIN_QUERY_ROOT,
  USER_GONE_MESSAGE,
  adminUserMutationKey,
  updateAdminUserState,
} from "../api";

export function useUpdateAdminUserState(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  // The whole "admin" prefix: the owner's listings change visibility too.
  const refreshAdminData = () => queryClient.invalidateQueries({ queryKey: [ADMIN_QUERY_ROOT] });

  return useMutation({
    mutationKey: adminUserMutationKey(id),
    mutationFn: (isActive: boolean) => updateAdminUserState(id, isActive),
    retry: false,
    // Returning the promise keeps the row locked until the refetch shows the new state.
    onSuccess: refreshAdminData,
    onError: async (error, isActive) => {
      if (isNotFound(error)) {
        showToast(USER_GONE_MESSAGE, { variant: "error" });
        await refreshAdminData();
        return;
      }
      // A failed suspension is shown in its dialog; a reactivation has none.
      if (isActive) {
        showToast("Couldn't reactivate the account. Please try again.", { variant: "error" });
      }
    },
  });
}
