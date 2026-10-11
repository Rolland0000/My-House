import { ApiError } from "../../shared/api/client";

export function isPhotoRequired(error: unknown): boolean {
  return error instanceof ApiError && error.status === 409 && error.code === "PHOTO_REQUIRED";
}
