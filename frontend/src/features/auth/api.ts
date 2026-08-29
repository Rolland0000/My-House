import { apiPost, apiPut } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type OtpVerifyToken = components["schemas"]["OtpVerifyTokenDto"];
export type UpdateMePayload = components["schemas"]["UpdateMeDto"];
export type User = components["schemas"]["UserDto"];

export function requestOtp(email: string): Promise<{ data: { message: string } }> {
  return apiPost("/api/v1/auth/otp/request", { email });
}

export function verifyOtp(email: string, code: string): Promise<{ data: OtpVerifyToken }> {
  return apiPost("/api/v1/auth/otp/verify", { email, code });
}

export function refreshSession(): Promise<{ data: { access_token: string } }> {
  return apiPost("/api/v1/auth/refresh");
}

export function logout(): Promise<void> {
  return apiPost("/api/v1/auth/logout");
}

export function updateMe(payload: UpdateMePayload): Promise<{ data: User }> {
  return apiPut("/api/v1/users/me", payload);
}
