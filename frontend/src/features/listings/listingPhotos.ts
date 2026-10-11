import { ApiError } from "../../shared/api/client";
import { MAX_IMAGE_SIZE_BYTES, MAX_PHOTOS_PER_LISTING } from "../../shared/api/constants";
import { imageFileProblem, type ImageFileProblem } from "../../shared/utils/imageFileCheck";

export const MAX_IMAGE_SIZE_MB = MAX_IMAGE_SIZE_BYTES / (1024 * 1024);

export const FORMAT_MESSAGE = "Unsupported format. Choose a JPEG, PNG, or WebP image.";
export const SIZE_MESSAGE = `Image too large. Maximum size is ${MAX_IMAGE_SIZE_MB} MB.`;
export const QUOTA_MESSAGE = `This listing already has ${MAX_PHOTOS_PER_LISTING} photos.`;
export const LISTING_GONE_MESSAGE = "This listing no longer exists.";
export const GENERIC_UPLOAD_MESSAGE = "Upload failed. Please try again.";
export const COVER_REQUIRED_MESSAGE = "Choose another cover before deleting this photo.";

export const PROBLEM_MESSAGES: Record<ImageFileProblem, string> = {
  format: FORMAT_MESSAGE,
  size: SIZE_MESSAGE,
};

export interface PhotoSelectionPlan {
  rejected: { file: File; problem: ImageFileProblem }[];
  toUpload: File[];
  skippedCount: number;
}

export function freePhotoSlots(photoCount: number, inFlightCount: number): number {
  return Math.max(0, MAX_PHOTOS_PER_LISTING - photoCount - inFlightCount);
}

/** Rejected files don't take a slot; valid ones fill the free slots in selection order. */
export function planPhotoSelection(files: File[], freeSlots: number): PhotoSelectionPlan {
  const rejected: PhotoSelectionPlan["rejected"] = [];
  const valid: File[] = [];
  for (const file of files) {
    const problem = imageFileProblem(file);
    if (problem) rejected.push({ file, problem });
    else valid.push(file);
  }
  const toUpload = valid.slice(0, freeSlots);
  return { rejected, toUpload, skippedCount: valid.length - toUpload.length };
}

/** The 413 is matched on status: nginx can answer it without the JSON error body. */
export function photoUploadErrorMessage(error: unknown): string {
  if (!(error instanceof ApiError)) return GENERIC_UPLOAD_MESSAGE;
  if (error.status === 413) return SIZE_MESSAGE;
  if (error.code === "INVALID_FILE") return FORMAT_MESSAGE;
  if (error.code === "MEDIA_QUOTA_EXCEEDED") return QUOTA_MESSAGE;
  if (error.status === 404) return LISTING_GONE_MESSAGE;
  return GENERIC_UPLOAD_MESSAGE;
}

export function isCoverRequiredError(error: unknown): boolean {
  return error instanceof ApiError && error.code === "COVER_PHOTO_REQUIRED";
}
