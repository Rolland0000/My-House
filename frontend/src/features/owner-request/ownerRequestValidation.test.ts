import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import { MAX_ID_IMAGE_SIZE_BYTES, MAX_ID_PDF_SIZE_BYTES } from "../../shared/api/constants";
import {
  GENERIC_ERROR_MESSAGE,
  IMAGE_FORMAT_MESSAGE,
  IMAGE_SIZE_MESSAGE,
  PDF_FORMAT_MESSAGE,
  PDF_SIZE_MESSAGE,
  preCheckImage,
  preCheckPdf,
  serverMessage,
} from "./ownerRequestValidation";

function file(bytes: number, type: string, name = "document"): File {
  return new File([new Uint8Array(bytes)], name, { type });
}

describe("preCheckImage", () => {
  it("accepts a valid image under the size limit", () => {
    expect(preCheckImage(file(1024, "image/jpeg"))).toBeNull();
  });

  it("rejects an unsupported MIME type", () => {
    expect(preCheckImage(file(1024, "application/pdf"))).toBe(IMAGE_FORMAT_MESSAGE);
  });

  it("rejects a file over the size limit", () => {
    expect(preCheckImage(file(MAX_ID_IMAGE_SIZE_BYTES + 1, "image/png"))).toBe(IMAGE_SIZE_MESSAGE);
  });
});

describe("preCheckPdf", () => {
  it("accepts a valid PDF under the size limit", () => {
    expect(preCheckPdf(file(1024, "application/pdf"))).toBeNull();
  });

  it("rejects a non-PDF file", () => {
    expect(preCheckPdf(file(1024, "image/jpeg"))).toBe(PDF_FORMAT_MESSAGE);
  });

  it("rejects a file over the size limit", () => {
    expect(preCheckPdf(file(MAX_ID_PDF_SIZE_BYTES + 1, "application/pdf"))).toBe(PDF_SIZE_MESSAGE);
  });
});

describe("serverMessage", () => {
  it("falls back to a generic message for a non-ApiError", () => {
    expect(serverMessage(new Error("network down"))).toBe(GENERIC_ERROR_MESSAGE);
  });

  it("maps OWNER_REQUEST_ALREADY_PENDING to an actionable message", () => {
    expect(serverMessage(new ApiError(409, "OWNER_REQUEST_ALREADY_PENDING", "Conflict"))).toContain(
      "already have a pending request"
    );
  });

  it("maps INVALID_DOCUMENT to the shape-guidance message", () => {
    expect(serverMessage(new ApiError(422, "INVALID_DOCUMENT", "Invalid document"))).toContain(
      "front and back"
    );
  });

  it("maps PAYLOAD_TOO_LARGE to a size message", () => {
    expect(serverMessage(new ApiError(413, "PAYLOAD_TOO_LARGE", "Too large"))).toContain("exceeds");
  });

  it("never surfaces the server's raw text for an unmapped code", () => {
    const raw = "Some backend validation message";
    expect(serverMessage(new ApiError(500, "INTERNAL_ERROR", raw))).toBe(GENERIC_ERROR_MESSAGE);
  });
});
