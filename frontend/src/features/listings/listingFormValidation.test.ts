import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import {
  GENERIC_ERROR_MESSAGE,
  ROLE_NOT_ACTIVE_MESSAGE,
  VALIDATION_FAILED_MESSAGE,
  integerFieldRule,
  normalizePlaceName,
  placeNameFieldRule,
  requestErrorMessage,
  serverFieldToFormField,
  toCreateListingPayload,
  type ListingFormValues,
} from "./listingFormValidation";

describe("normalizePlaceName", () => {
  it("trims leading and trailing spaces", () => {
    expect(normalizePlaceName("  Dakar ")).toBe("Dakar");
  });

  it("collapses a run of internal whitespace to a single space", () => {
    expect(normalizePlaceName("Plateau   Nord")).toBe("Plateau Nord");
  });

  it("collapses tabs, newlines, and non-breaking spaces", () => {
    expect(normalizePlaceName("Plateau\t\n Nord")).toBe("Plateau Nord");
  });

  it("returns an empty string for a value made only of whitespace", () => {
    expect(normalizePlaceName("     \t ")).toBe("");
  });
});

function baseValues(overrides: Partial<ListingFormValues> = {}): ListingFormValues {
  return {
    title: "Studio meuble Plateau",
    description: "Un bel appartement bien situe pres du centre-ville.",
    type: "studio",
    price: 150000,
    surfaceM2: 35,
    rooms: 1,
    city: "  Dakar ",
    neighborhood: "Plateau   Nord",
    ...overrides,
  };
}

describe("toCreateListingPayload", () => {
  it("trims text fields and normalizes city and neighborhood", () => {
    const payload = toCreateListingPayload(baseValues());
    expect(payload.title).toBe("Studio meuble Plateau");
    expect(payload.city).toBe("Dakar");
    expect(payload.neighborhood).toBe("Plateau Nord");
  });

  it("omits surface_m2 and rooms when left empty (NaN)", () => {
    const payload = toCreateListingPayload(baseValues({ surfaceM2: NaN, rooms: NaN }));
    expect(payload).not.toHaveProperty("surface_m2");
    expect(payload).not.toHaveProperty("rooms");
  });

  it("includes surface_m2 and rooms when provided", () => {
    const payload = toCreateListingPayload(baseValues());
    expect(payload.surface_m2).toBe(35);
    expect(payload.rooms).toBe(1);
  });

  it("never emits an owner_id field", () => {
    const payload = toCreateListingPayload(baseValues());
    expect(payload).not.toHaveProperty("owner_id");
  });
});

describe("integerFieldRule", () => {
  const rule = integerFieldRule(1, 100, " m²");

  it("accepts an empty (NaN) value", () => {
    expect(rule.validate(NaN)).toBe(true);
  });

  it("accepts an integer", () => {
    expect(rule.validate(35)).toBe(true);
  });

  it("rejects a non-integer value", () => {
    expect(rule.validate(35.5)).toBe("Must be a whole number.");
  });

  it("formats bound messages with the given unit", () => {
    expect(rule.min.message).toBe("At least 1 m².");
    expect(rule.max.message).toBe("At most 100 m².");
  });
});

describe("placeNameFieldRule", () => {
  const rule = placeNameFieldRule("City");

  it("requires a value", () => {
    expect(rule.required).toBe("City is required.");
  });

  it("rejects a value made only of whitespace", () => {
    expect(rule.validate("   ")).toBe("City is required.");
  });

  it("accepts a normal value", () => {
    expect(rule.validate("Dakar")).toBe(true);
  });
});

describe("serverFieldToFormField", () => {
  it("maps a field that matches the form's own name as-is", () => {
    expect(serverFieldToFormField("title")).toBe("title");
  });

  it("maps the one snake_case field that differs from its form name", () => {
    expect(serverFieldToFormField("surface_m2")).toBe("surfaceM2");
  });

  it("returns undefined for a field the form doesn't have", () => {
    expect(serverFieldToFormField("owner_id")).toBeUndefined();
  });
});

describe("requestErrorMessage", () => {
  it("maps VALIDATION_FAILED to the field-fix banner message", () => {
    expect(requestErrorMessage(new ApiError(422, "VALIDATION_FAILED", "Invalid"))).toBe(
      VALIDATION_FAILED_MESSAGE
    );
  });

  it("maps FORBIDDEN to the role-not-active message", () => {
    expect(requestErrorMessage(new ApiError(403, "FORBIDDEN", "Forbidden"))).toBe(
      ROLE_NOT_ACTIVE_MESSAGE
    );
  });

  it("falls back to a generic message for an unmapped code", () => {
    expect(requestErrorMessage(new ApiError(500, "INTERNAL_ERROR", "Boom"))).toBe(
      GENERIC_ERROR_MESSAGE
    );
  });

  it("falls back to a generic message for a non-ApiError", () => {
    expect(requestErrorMessage(new Error("network down"))).toBe(GENERIC_ERROR_MESSAGE);
  });
});
