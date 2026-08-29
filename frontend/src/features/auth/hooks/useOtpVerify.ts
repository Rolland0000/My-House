import { useMutation } from "@tanstack/react-query";
import { verifyOtp } from "../api";
import { useAuth } from "./useAuth";

interface VerifyOtpParams {
  email: string;
  code: string;
}

export function useOtpVerify() {
  const { setSession } = useAuth();

  return useMutation({
    mutationFn: ({ email, code }: VerifyOtpParams) => verifyOtp(email, code),
    retry: false,
    onSuccess: ({ data }) => setSession(data.access_token),
  });
}
