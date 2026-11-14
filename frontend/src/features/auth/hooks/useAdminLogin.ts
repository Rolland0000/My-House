import { useMutation } from "@tanstack/react-query";
import { adminLogin } from "../api";
import { useAuth } from "./useAuth";

interface AdminLoginParams {
  email: string;
  password: string;
}

export function useAdminLogin() {
  const { setSession } = useAuth();

  return useMutation({
    mutationFn: ({ email, password }: AdminLoginParams) => adminLogin(email, password),
    retry: false,
    onSuccess: ({ data }) => setSession(data.access_token),
  });
}
