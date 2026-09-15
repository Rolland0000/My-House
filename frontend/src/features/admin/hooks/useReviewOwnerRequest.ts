import { useMutation, useQueryClient } from "@tanstack/react-query";
import { reviewOwnerRequest } from "../api";
import type { ReviewDecision } from "../api";

interface ReviewOwnerRequestInput {
  id: string;
  status: ReviewDecision;
  adminNote?: string;
}

export function useReviewOwnerRequest() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, status, adminNote }: ReviewOwnerRequestInput) =>
      reviewOwnerRequest(id, status, adminNote),
    retry: false,
    // Refetch on every outcome, including a 409: another admin's concurrent
    // review needs the current status to show up, not just a successful one.
    onSettled: () =>
      queryClient.invalidateQueries({
        predicate: (query) =>
          query.queryKey[0] === "admin" && query.queryKey[1] === "owner-requests",
      }),
  });
}
