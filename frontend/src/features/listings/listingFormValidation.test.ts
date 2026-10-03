import { describe, expect, it } from "vitest";
import { ApiError } from "../../shared/api/client";
import type { ListingDetail } from "./api";
import {
  GENERIC_ERROR_MESSAGE,
  ROLE_NOT_ACTIVE_MESSAGE,
  VALIDATION_FAILED_MESSAGE,
  integerFieldRule,
  listingDetailToFormValues,
  normalizePlaceName,
  placeNameFieldRule,
  requestErrorMessage,
  serverFieldToFormField,
  toListingPayload,
  trimmedTextFieldRule,
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

describe("toListingPayload", () => {
  it("trims text fields and normalizes city and neighborhood", () => {
    const payload = toListingPayload(baseValues());
    expect(payload.title).toBe("Studio meuble Plateau");
    expect(payload.city).toBe("Dakar");
    expect(payload.neighborhood).toBe("Plateau Nord");
  });

  it("omits surface_m2 and rooms when left empty (NaN)", () => {
    const payload = toListingPayload(baseValues({ surfaceM2: NaN, rooms: NaN }));
    expect(payload).not.toHaveProperty("surface_m2");
    expect(payload).not.toHaveProperty("rooms");
  });

  it("includes surface_m2 and rooms when provided", () => {
    const payload = toListingPayload(baseValues());
    expect(payload.surface_m2).toBe(35);
    expect(payload.rooms).toBe(1);
  });

  it("never emits an owner_id field", () => {
    const payload = toListingPayload(baseValues());
    expect(payload).not.toHaveProperty("owner_id");
  });
});

function baseDetail(overrides: Partial<ListingDetail> = {}): ListingDetail {
  return {
    id: "listing-1",
    title: "Studio meuble Plateau",
    description: "Un bel appartement bien situe pres du centre-ville.",
    type: "studio",
    price: 150000,
    surface_m2: 35,
    rooms: 1,
    city: "Dakar",
    neighborhood: "Plateau",
    status: "available",
    published_at: null,
    created_at: "2026-01-01T00:00:00Z",
    media: [],
    owner: { id: "owner-1", first_name: "Awa", last_name: "Diop" },
    ...overrides,
  };
}

describe("listingDetailToFormValues", () => {
  it("copies text fields and type as-is", () => {
    const values = listingDetailToFormValues(baseDetail());
    expect(values).toMatchObject({
      title: "Studio meuble Plateau",
      description: "Un bel appartement bien situe pres du centre-ville.",
      type: "studio",
      city: "Dakar",
      neighborhood: "Plateau",
      surfaceM2: 35,
      rooms: 1,
    });
  });

  it("keeps a whole price and rounds a float price to a whole number", () => {
    expect(listingDetailToFormValues(baseDetail()).price).toBe(150000);
    expect(listingDetailToFormValues(baseDetail({ price: 150000.0000001 })).price).toBe(150000);
  });

  it("maps null surface and rooms to NaN", () => {
    const values = listingDetailToFormValues(baseDetail({ surface_m2: null, rooms: null }));
    expect(values.surfaceM2).toBeNaN();
    expect(values.rooms).toBeNaN();
  });

  it("maps a null neighborhood to an empty string", () => {
    expect(listingDetailToFormValues(baseDetail({ neighborhood: null })).neighborhood).toBe("");
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

describe("trimmedTextFieldRule", () => {
  const rule = trimmedTextFieldRule("Title", 5, 10);

  it("requires a value", () => {
    expect(rule.required).toBe("Title is required.");
  });

  it("rejects a value made only of whitespace", () => {
    expect(rule.validate("     ")).toBe("Title is required.");
  });

  it("checks both bounds on the trimmed length", () => {
    expect(rule.validate("  abcd  ")).toBe("5 characters minimum.");
    expect(rule.validate("  abcde  ")).toBe(true);
    expect(rule.validate("a".repeat(10))).toBe(true);
    expect(rule.validate("a".repeat(11))).toBe("10 characters maximum.");
  });

  it("counts characters, not UTF-16 units", () => {
    expect(rule.validate("😀".repeat(5))).toBe(true);
    expect(rule.validate("😀".repeat(10))).toBe(true);
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
