export const OTP_CODE_LENGTH = 6;

// Fallback only: 429 responses carry the real window in `Retry-After`
// (ApiError.retryAfterSeconds), which the forms use when present.
export const OTP_RATE_LIMIT_COOLDOWN_SECONDS = 60;

// Mirrors the server-side bounds in backend/src/shared/validation.rs.
export const MAX_NAME_LENGTH = 100;
export const MAX_PHONE_LENGTH = 30;

export const OTP_RESEND_INITIAL_COOLDOWN_SECONDS = 45;
