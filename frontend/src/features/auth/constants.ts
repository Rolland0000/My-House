export const OTP_CODE_LENGTH = 6;

// The 429 response carries no Retry-After header (backend/src/shared/errors.rs),
// so this mirrors OTP_RATE_LIMIT_SECONDS from backend/.env.example as a fixed
// local estimate rather than a value read from the response.
export const OTP_RATE_LIMIT_COOLDOWN_SECONDS = 60;

export const OTP_RESEND_INITIAL_COOLDOWN_SECONDS = 45;
