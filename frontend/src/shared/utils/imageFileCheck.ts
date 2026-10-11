import { ACCEPTED_IMAGE_TYPES, MAX_IMAGE_SIZE_BYTES } from "../api/constants";

export type ImageFileProblem = "format" | "size";

/** Local check only: the server re-checks the actual bytes and can still
 *  reject a file that passes here. */
export function imageFileProblem(file: File): ImageFileProblem | null {
  if (!ACCEPTED_IMAGE_TYPES.includes(file.type)) return "format";
  if (file.size > MAX_IMAGE_SIZE_BYTES) return "size";
  return null;
}
