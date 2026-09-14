import type { BadgeTone } from "../../shared/components";
import type { components } from "../../shared/api/types";

type OwnerRequestStatus = components["schemas"]["OwnerRequestStatus"];

const STATUS_BADGE: Record<OwnerRequestStatus, { tone: BadgeTone; label: string }> = {
  pending: { tone: "warning", label: "Pending" },
  approved: { tone: "success", label: "Approved" },
  rejected: { tone: "error", label: "Rejected" },
};

export function ownerRequestStatusBadge(status: OwnerRequestStatus) {
  return STATUS_BADGE[status];
}
