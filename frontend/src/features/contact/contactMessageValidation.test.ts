import { describe, expect, it } from "vitest";
import {
  MESSAGE_MAX_CHARS,
  NAME_MAX_CHARS,
  validateContactMessage,
  type ContactMessageFields,
} from "./contactMessageValidation";

const VALID: ContactMessageFields = {
  name: "Awa",
  email: "awa@example.com",
  message: "I would like some help.",
};

describe("validateContactMessage", () => {
  it("accepts a complete message", () => {
    expect(validateContactMessage(VALID)).toEqual({});
  });

  it("asks for a name, and bounds its length", () => {
    expect(validateContactMessage({ ...VALID, name: "  " }).name).toBeDefined();
    expect(
      validateContactMessage({ ...VALID, name: "a".repeat(NAME_MAX_CHARS) }).name
    ).toBeUndefined();
    expect(
      validateContactMessage({ ...VALID, name: "a".repeat(NAME_MAX_CHARS + 1) }).name
    ).toBeDefined();
  });

  it("rejects an address without a domain", () => {
    for (const email of ["", "awa", "awa@", "awa@example", "a b@example.com"]) {
      expect(validateContactMessage({ ...VALID, email }).email).toBeDefined();
    }
  });

  it("bounds the message length in characters, after trimming", () => {
    expect(validateContactMessage({ ...VALID, message: "   short   " }).message).toBeDefined();
    expect(
      validateContactMessage({ ...VALID, message: "é".repeat(MESSAGE_MAX_CHARS) }).message
    ).toBeUndefined();
    expect(
      validateContactMessage({ ...VALID, message: "é".repeat(MESSAGE_MAX_CHARS + 1) }).message
    ).toBeDefined();
  });
});
