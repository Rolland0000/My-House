import { useMutation } from "@tanstack/react-query";
import { sendContactMessage, type ContactMessagePayload } from "../api";

export function useSendContactMessage() {
  return useMutation({
    mutationFn: (payload: ContactMessagePayload) => sendContactMessage(payload),
    retry: false,
  });
}
