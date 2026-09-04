import { useMutation, useQueryClient } from "@tanstack/react-query";
import { profileQueryKey, uploadAvatar } from "../api";

/** Writes the response into the profile cache, like `useUpdateProfile`: the
 *  server returns the updated record, and the url carries a fresh UUID on
 *  every upload, so nothing needs cache-busting. */
export function useUploadAvatar() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: uploadAvatar,
    retry: false,
    onSuccess: (response) => queryClient.setQueryData(profileQueryKey, response),
  });
}
