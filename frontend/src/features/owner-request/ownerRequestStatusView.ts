import type { BadgeTone } from "../../shared/components";
import type { OwnerRequest } from "./api";

export type OwnerRequestViewState = "no-request" | "pending" | "approved" | "rejected";

export interface OwnerRequestStatusView {
  state: OwnerRequestViewState;
  badgeTone: BadgeTone;
  badgeLabel: string;
  title: string;
  description: string;
  showCta: boolean;
  adminNote: string | null;
}

type BaseView = Omit<OwnerRequestStatusView, "adminNote">;

const VIEWS: Record<OwnerRequestViewState, BaseView> = {
  "no-request": {
    state: "no-request",
    badgeTone: "neutral",
    badgeLabel: "No request",
    title: "Become an owner",
    description: "Submit an owner request to start listing properties on MyHouse.",
    showCta: true,
  },
  pending: {
    state: "pending",
    badgeTone: "warning",
    badgeLabel: "Pending",
    title: "Your request is pending",
    description:
      "Our team is reviewing your information. You'll be notified by email once a decision is made.",
    showCta: false,
  },
  approved: {
    state: "approved",
    badgeTone: "success",
    badgeLabel: "Approved",
    title: "Your request was approved",
    description: "You now have owner access.",
    showCta: false,
  },
  rejected: {
    state: "rejected",
    badgeTone: "error",
    badgeLabel: "Rejected",
    title: "Your request was rejected",
    description: "You can review the note below and submit a new request.",
    showCta: true,
  },
};

/** `data` is `null` when the caller has never applied. */
export function ownerRequestStatusView(data: OwnerRequest | null): OwnerRequestStatusView {
  const base = data ? VIEWS[data.status] : VIEWS["no-request"];
  const adminNote = data?.status === "rejected" ? (data.admin_note ?? null) : null;
  return { ...base, adminNote };
}
