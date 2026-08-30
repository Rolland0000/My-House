import { useMutation } from "@tanstack/react-query";
import { requestOtp } from "../api";

export function useOtpRequest() {
  return useMutation({
    mutationFn: (email: string) => requestOtp(email),
    retry: false,
  });
}
