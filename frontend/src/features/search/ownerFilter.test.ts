import { describe, expect, it } from "vitest";
import { ownerFilterLabel } from "./ownerFilter";

const OWNER_ID = "3f2b8c1e-9a4d-4e6f-8b2a-1c5d7e9f0a3b";
const OTHER_ID = "9c1d2e3f-4a5b-4c6d-8e7f-0a1b2c3d4e5f";

function result(id: string, firstName: string | null, lastName: string | null) {
  return { owner: { id, first_name: firstName, last_name: lastName } };
}

describe("ownerFilterLabel", () => {
  it("names the owner from a matching result", () => {
    expect(ownerFilterLabel(OWNER_ID, [result(OWNER_ID, "Fatou", "Ndiaye")])).toBe(
      "Listings by Fatou Ndiaye"
    );
  });

  it("ignores results that belong to another owner", () => {
    expect(ownerFilterLabel(OWNER_ID, [result(OTHER_ID, "Moussa", "Diallo")])).toBe(
      "Listings by this owner"
    );
  });

  it("falls back when there is no result", () => {
    expect(ownerFilterLabel(OWNER_ID, [])).toBe("Listings by this owner");
  });

  it("falls back when the owner has no name", () => {
    expect(ownerFilterLabel(OWNER_ID, [result(OWNER_ID, null, " ")])).toBe(
      "Listings by this owner"
    );
  });
});
