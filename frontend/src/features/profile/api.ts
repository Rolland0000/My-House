import { apiGet, apiPut, apiUpload } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type Profile = components["schemas"]["UserDto"];
export type UpdateProfilePayload = components["schemas"]["UpdateMeDto"];

type UserResponse = components["schemas"]["UserResponse"];

/** Shared by the read hook and the mutation writing its result back. */
export const profileQueryKey = ["profile", "me"] as const;

export function getMe(): Promise<UserResponse> {
  return apiGet("/api/v1/users/me");
}

export function updateMe(payload: UpdateProfilePayload): Promise<UserResponse> {
  return apiPut("/api/v1/users/me", payload);
}

export function uploadAvatar(file: File): Promise<UserResponse> {
  const formData = new FormData();
  formData.append("file", file);
  return apiUpload("/api/v1/users/me/avatar", formData);
}
