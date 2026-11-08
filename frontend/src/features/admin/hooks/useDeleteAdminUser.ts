import { useMutation, useQueryClient } from "@tanstack/react-query";
import { isNotFound } from "../../../shared/api/client";
import { useToast } from "../../../shared/components";
import { withoutRow } from "../../../shared/utils/paginatedRows";
import {
  ADMIN_QUERY_ROOT,
  ADMIN_USERS_KEY,
  USER_GONE_MESSAGE,
  adminUserMutationKey,
  deleteAdminUser,
  type AdminUsersResult,
} from "../api";

/** Results are handled in the hook because the row unmounts, and `mutate()` callbacks don't fire after that. */
export function useDeleteAdminUser(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  function removeRow() {
    queryClient.setQueriesData<AdminUsersResult>({ queryKey: ADMIN_USERS_KEY }, (result) =>
      withoutRow(result, id)
    );
    // The whole "admin" prefix: a deleted account also takes its owner requests with it.
    void queryClient.invalidateQueries({ queryKey: [ADMIN_QUERY_ROOT] });
  }

  return useMutation({
    mutationKey: adminUserMutationKey(id),
    mutationFn: () => deleteAdminUser(id),
    retry: false,
    onSuccess: () => {
      removeRow();
      showToast("Account deleted.", { variant: "success" });
    },
    onError: (error) => {
      if (!isNotFound(error)) return;
      removeRow();
      showToast(USER_GONE_MESSAGE, { variant: "error" });
    },
  });
}
