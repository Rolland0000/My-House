import { useMutation, useQueryClient } from "@tanstack/react-query";
import { ownerRequestStatusQueryKey, submitOwnerRequest } from "../api";

export function useSubmitOwnerRequest() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: submitOwnerRequest,
    retry: false,
    onSuccess: (response) => queryClient.setQueryData(ownerRequestStatusQueryKey, response),
  });
}
