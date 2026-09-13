import { describe, expect, it } from "vitest";
import { ownerRequestStatusBadge } from "./ownerRequestStatusBadge";

describe("ownerRequestStatusBadge", () => {
  it("maps each status to its tone and label", () => {
    expect(ownerRequestStatusBadge("pending")).toEqual({ tone: "warning", label: "Pending" });
    expect(ownerRequestStatusBadge("approved")).toEqual({ tone: "success", label: "Approved" });
    expect(ownerRequestStatusBadge("rejected")).toEqual({ tone: "error", label: "Rejected" });
  });
});
