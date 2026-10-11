import { describe, expect, it } from "vitest";
import { MAX_IMAGE_SIZE_BYTES } from "../api/constants";
import { imageFileProblem } from "./imageFileCheck";

function file(bytes: number, type: string): File {
  return new File([new Uint8Array(bytes)], "photo", { type });
}

describe("imageFileProblem", () => {
  it.each(["image/jpeg", "image/png", "image/webp"])("accepts %s under the size limit", (type) => {
    expect(imageFileProblem(file(1024, type))).toBeNull();
  });

  it("rejects an unsupported type", () => {
    expect(imageFileProblem(file(1024, "image/gif"))).toBe("format");
  });

  it("rejects a file with no type", () => {
    expect(imageFileProblem(file(1024, ""))).toBe("format");
  });

  it("rejects a file over the size limit", () => {
    expect(imageFileProblem(file(MAX_IMAGE_SIZE_BYTES + 1, "image/png"))).toBe("size");
  });

  it("accepts a file exactly at the size limit", () => {
    expect(imageFileProblem(file(MAX_IMAGE_SIZE_BYTES, "image/png"))).toBeNull();
  });

  it("reports the format first when both checks fail", () => {
    expect(imageFileProblem(file(MAX_IMAGE_SIZE_BYTES + 1, "image/gif"))).toBe("format");
  });
});
