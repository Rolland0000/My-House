import { describe, expect, it } from "vitest";
import { ADMIN_VISIBILITY_BADGE } from "./adminListingVisibilityBadge";

describe("ADMIN_VISIBILITY_BADGE", () => {
  it("maps each visibility to its tone and label", () => {
    expect(ADMIN_VISIBILITY_BADGE).toEqual({
      draft: { tone: "warning", label: "Draft" },
      no_photo: { tone: "error", label: "No photo" },
      public: { tone: "success", label: "Public" },
      hidden: { tone: "neutral", label: "Hidden" },
    });
  });
});
