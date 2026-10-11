import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import { MAX_IMAGE_SIZE_BYTES } from "../../shared/api/constants";
import {
  FORMAT_MESSAGE,
  GENERIC_ERROR_MESSAGE,
  SIZE_MESSAGE,
  preCheck,
  serverMessage,
} from "./avatarValidation";

function file(bytes: number, type = "image/png"): File {
  return new File([new Uint8Array(bytes)], "avatar.png", { type });
}

describe("preCheck", () => {
  it("returns no message for a valid image", () => {
    expect(preCheck(file(1024, "image/jpeg"))).toBeNull();
  });

  it("maps a format problem to the format message", () => {
    expect(preCheck(file(1024, "image/gif"))).toBe(FORMAT_MESSAGE);
  });

  it("maps a size problem to the size message", () => {
    expect(preCheck(file(MAX_IMAGE_SIZE_BYTES + 1))).toBe(SIZE_MESSAGE);
  });
});

describe("serverMessage", () => {
  it("falls back to a generic message for a non-ApiError", () => {
    expect(serverMessage(new Error("network down"))).toBe(GENERIC_ERROR_MESSAGE);
  });

  it("maps PAYLOAD_TOO_LARGE to the size message", () => {
    expect(serverMessage(new ApiError(413, "PAYLOAD_TOO_LARGE", "Payload too large"))).toBe(
      SIZE_MESSAGE
    );
  });

  it("maps INVALID_FILE to the format message", () => {
    expect(serverMessage(new ApiError(400, "INVALID_FILE", "Invalid file"))).toBe(FORMAT_MESSAGE);
  });

  it("maps BAD_REQUEST to an unreadable-file message", () => {
    expect(serverMessage(new ApiError(400, "BAD_REQUEST", "Bad request"))).toContain("Unreadable");
  });

  it("never surfaces the server's raw text for an unmapped code", () => {
    const raw = "Some backend validation message";
    expect(serverMessage(new ApiError(500, "INTERNAL_ERROR", raw))).toBe(GENERIC_ERROR_MESSAGE);
  });
});
