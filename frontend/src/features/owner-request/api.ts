import { apiGet, apiUpload } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type OwnerRequest = components["schemas"]["OwnerRequestDto"];
type OwnerRequestResponse = components["schemas"]["OwnerRequestResponse"];

export const ownerRequestStatusQueryKey = ["owner-request", "status"] as const;

export function submitOwnerRequest(formData: FormData): Promise<OwnerRequestResponse> {
  return apiUpload("/api/v1/owner-requests", formData);
}

export function getOwnerRequestStatus(): Promise<OwnerRequestResponse> {
  return apiGet("/api/v1/users/me/owner-request");
}
