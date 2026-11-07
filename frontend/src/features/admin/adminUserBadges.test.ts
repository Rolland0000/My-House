import { describe, expect, it } from "vitest";
import { userStateBadge } from "./adminUserBadges";

describe("userStateBadge", () => {
  it("maps is_active to the account state", () => {
    expect(userStateBadge(true)).toEqual({ tone: "success", label: "Active" });
    expect(userStateBadge(false)).toEqual({ tone: "error", label: "Suspended" });
  });
});
