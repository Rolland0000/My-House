import { useMutation } from "@tanstack/react-query";
import { updateMe } from "../api";

export function useUpdateProfile() {
  return useMutation({
    mutationFn: updateMe,
    retry: false,
  });
}
