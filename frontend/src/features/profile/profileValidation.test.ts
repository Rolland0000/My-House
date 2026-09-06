import { describe, expect, it } from "vitest";
import { MAX_NAME_LENGTH, MAX_PHONE_LENGTH } from "../../shared/api/constants";
import { REQUIRED_MESSAGE, serverFieldError, validate } from "./profileValidation";

describe("validate", () => {
  it("passes with a last name and a phone", () => {
    expect(validate("", "Doe", "0700000000")).toEqual({});
  });

  it("requires a last name", () => {
    expect(validate("Jane", "", "0700000000")).toEqual({ lastName: REQUIRED_MESSAGE });
  });

  it("requires a phone", () => {
    expect(validate("Jane", "Doe", "")).toEqual({ phone: REQUIRED_MESSAGE });
  });

  it("rejects a first name over the max length", () => {
    const errors = validate("a".repeat(MAX_NAME_LENGTH + 1), "Doe", "0700000000");
    expect(errors.firstName).toBeDefined();
  });

  it("rejects a phone over the max length", () => {
    const errors = validate("Jane", "Doe", "0".repeat(MAX_PHONE_LENGTH + 1));
    expect(errors.phone).toBeDefined();
  });
});

describe("serverFieldError", () => {
  it("maps a last_name 'required' error to the last name field", () => {
    const result = serverFieldError("last_name is a required field");
    expect(result).toEqual({ field: "lastName", text: REQUIRED_MESSAGE });
  });

  it("maps a phone 'at most' error to a max-length message on the phone field", () => {
    const result = serverFieldError(`phone must be at most ${MAX_PHONE_LENGTH} characters`);
    expect(result?.field).toBe("phone");
    expect(result?.text).toContain(String(MAX_PHONE_LENGTH));
  });

  it("returns null for a message naming no known field", () => {
    expect(serverFieldError("something went wrong")).toBeNull();
  });

  it("returns null for a recognised field with unrecognised wording", () => {
    // Guards against a silent regression if the backend rewords its message —
    // an unmatched rule must fall through to the generic banner, not a
    // mislabelled field error.
    expect(serverFieldError("last_name looks suspicious")).toBeNull();
  });
});
