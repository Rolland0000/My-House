import { useMutation, useQueryClient } from "@tanstack/react-query";
import { profileQueryKey, updateMe } from "../api";

/** Writes the response straight into the profile cache: the server already
 *  returns the updated record, so no refetch is needed to show it. */
export function useUpdateProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: updateMe,
    retry: false,
    onSuccess: (response) => queryClient.setQueryData(profileQueryKey, response),
  });
}
