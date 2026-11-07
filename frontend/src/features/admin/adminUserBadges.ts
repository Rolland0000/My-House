import type { BadgeTone } from "../../shared/components";
import type { AccountState, UserRole } from "./api";

interface BadgeSpec {
  tone: BadgeTone;
  label: string;
}

export const ROLE_BADGE: Record<UserRole, BadgeSpec> = {
  seeker: { tone: "neutral", label: "Seeker" },
  owner: { tone: "brass", label: "Owner" },
  admin: { tone: "warning", label: "Admin" },
};

export const STATE_BADGE: Record<AccountState, BadgeSpec> = {
  active: { tone: "success", label: "Active" },
  suspended: { tone: "error", label: "Suspended" },
};

export function userStateBadge(isActive: boolean) {
  return STATE_BADGE[isActive ? "active" : "suspended"];
}
