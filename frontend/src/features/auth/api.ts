import { apiPost, apiPut } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type OtpVerifyToken = components["schemas"]["OtpVerifyTokenDto"];
export type RegisterPayload = components["schemas"]["RegisterDto"];
export type UpdateMePayload = components["schemas"]["UpdateMeDto"];
export type User = components["schemas"]["UserDto"];

type OtpRequestResponse = components["schemas"]["OtpRequestResponse"];
type OtpVerifyResponse = components["schemas"]["OtpVerifyResponse"];
type RefreshResponse = components["schemas"]["RefreshResponse"];
type RegisterResponse = components["schemas"]["RegisterResponse"];
type UserResponse = components["schemas"]["UserResponse"];

export function requestOtp(email: string): Promise<OtpRequestResponse> {
  return apiPost("/api/v1/auth/otp/request", { email });
}

export function verifyOtp(email: string, code: string): Promise<OtpVerifyResponse> {
  return apiPost("/api/v1/auth/otp/verify", { email, code });
}

/** Creates the account from the ticket returned by [`verifyOtp`]. */
export function registerAccount(payload: RegisterPayload): Promise<RegisterResponse> {
  return apiPost("/api/v1/auth/register", payload);
}

export function refreshSession(): Promise<RefreshResponse> {
  return apiPost("/api/v1/auth/refresh");
}

export function logout(): Promise<void> {
  return apiPost("/api/v1/auth/logout");
}

export function updateMe(payload: UpdateMePayload): Promise<UserResponse> {
  return apiPut("/api/v1/users/me", payload);
}
