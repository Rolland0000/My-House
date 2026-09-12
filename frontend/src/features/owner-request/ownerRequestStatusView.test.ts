import { describe, expect, it } from "vitest";
import type { OwnerRequest } from "./api";
import { ownerRequestStatusView } from "./ownerRequestStatusView";

function request(status: OwnerRequest["status"], adminNote: string | null = null): OwnerRequest {
  return {
    id: "5b1f7e2a-3c4d-4e5f-8a9b-0c1d2e3f4a5b",
    status,
    created_at: "2026-09-01T10:00:00Z",
    reviewed_at: null,
    admin_note: adminNote,
  };
}

describe("ownerRequestStatusView", () => {
  it("returns the no-request state with a CTA when data is null", () => {
    const view = ownerRequestStatusView(null);
    expect(view.state).toBe("no-request");
    expect(view.showCta).toBe(true);
    expect(view.adminNote).toBeNull();
  });

  it("returns the pending state with no CTA", () => {
    const view = ownerRequestStatusView(request("pending"));
    expect(view.state).toBe("pending");
    expect(view.badgeTone).toBe("warning");
    expect(view.showCta).toBe(false);
  });

  it("returns the approved state with no CTA", () => {
    const view = ownerRequestStatusView(request("approved"));
    expect(view.state).toBe("approved");
    expect(view.badgeTone).toBe("success");
    expect(view.showCta).toBe(false);
  });

  it("returns the rejected state with a CTA and the admin note", () => {
    const view = ownerRequestStatusView(request("rejected", "Blurry ID photo"));
    expect(view.state).toBe("rejected");
    expect(view.badgeTone).toBe("error");
    expect(view.showCta).toBe(true);
    expect(view.adminNote).toBe("Blurry ID photo");
  });

  it("omits the admin note for a rejected request that has none", () => {
    expect(ownerRequestStatusView(request("rejected")).adminNote).toBeNull();
  });
});
