import { useMutation } from "@tanstack/react-query";
import { registerAccount } from "../api";
import { useAuth } from "./useAuth";

export function useRegister() {
  const { setSession } = useAuth();

  return useMutation({
    mutationFn: registerAccount,
    retry: false,
    onSuccess: ({ data }) => setSession(data.access_token),
  });
}
