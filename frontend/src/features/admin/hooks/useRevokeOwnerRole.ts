import { useMutation, useQueryClient } from "@tanstack/react-query";
import { isNotAnOwner, isNotFound } from "../../../shared/api/client";
import { useToast } from "../../../shared/components";
import { ADMIN_QUERY_ROOT, USER_GONE_MESSAGE, adminUserMutationKey, revokeOwnerRole } from "../api";

export function useRevokeOwnerRole(id: string) {
  const queryClient = useQueryClient();
  const { showToast } = useToast();

  // The whole "admin" prefix: the owner's listings become hidden too.
  const refreshAdminData = () => queryClient.invalidateQueries({ queryKey: [ADMIN_QUERY_ROOT] });

  return useMutation({
    mutationKey: adminUserMutationKey(id),
    mutationFn: () => revokeOwnerRole(id),
    retry: false,
    // Awaiting the refetch keeps the row locked until it shows the seeker badge.
    onSuccess: async () => {
      await refreshAdminData();
      showToast("Owner role revoked.", { variant: "success" });
    },
    // After either refetch the row no longer offers the action, so a toast reports it.
    onError: async (error) => {
      if (isNotFound(error)) {
        showToast(USER_GONE_MESSAGE, { variant: "error" });
        await refreshAdminData();
      } else if (isNotAnOwner(error)) {
        showToast("This user is no longer an owner.", { variant: "error" });
        await refreshAdminData();
      }
    },
  });
}
