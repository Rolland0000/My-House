// Field bounds mirroring `backend/src/shared/validation.rs` — the server stays
// the authority; these only spare the user a round-trip.
export const MAX_NAME_LENGTH = 100;
export const MAX_PHONE_LENGTH = 30;

// Mirrors `MAX_IMAGE_SIZE_BYTES` in `backend/src/shared/file_validation.rs`.
export const MAX_AVATAR_SIZE_BYTES = 5 * 1024 * 1024;
export const ACCEPTED_AVATAR_TYPES = ["image/jpeg", "image/png", "image/webp"];
