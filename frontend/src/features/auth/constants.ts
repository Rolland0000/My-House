export const OTP_CODE_LENGTH = 6;

// Fallback only: 429 responses carry the real window in `Retry-After`
// (ApiError.retryAfterSeconds), which the forms use when present.
export const OTP_RATE_LIMIT_COOLDOWN_SECONDS = 60;

export const OTP_RESEND_INITIAL_COOLDOWN_SECONDS = 45;

export const ACCOUNT_SUSPENDED_MESSAGE = "Your account has been suspended.";

// Fallback only: the 429 for a locked admin account carries the real wait in `Retry-After`.
export const ADMIN_LOCKED_FALLBACK_SECONDS = 15 * 60;
