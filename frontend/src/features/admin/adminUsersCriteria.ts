import { parsePageParam } from "../../shared/utils/pageParam";
import { ROLE_BADGE, STATE_BADGE } from "./adminUserBadges";
import type { AccountState, ListAdminUsersParams, UserRole } from "./api";

/** Same limit as the API, counted in characters. */
export const MAX_SEARCH_CHARS = 100;

export const ROLES = Object.keys(ROLE_BADGE) as UserRole[];
export const STATES = Object.keys(STATE_BADGE) as AccountState[];

export interface AdminUsersCriteria {
  q: string;
  role: UserRole | "";
  state: AccountState | "";
  page: number;
}

function pickKnown<T extends string>(known: readonly T[], raw: string | null): T | "" {
  return known.find((value) => value === raw) ?? "";
}

/** Reads the criteria from the URL; an invalid value counts as absent. */
export function parseAdminUsersCriteria(params: URLSearchParams): AdminUsersCriteria {
  const q = params.get("q")?.trim() ?? "";
  return {
    q: [...q].length <= MAX_SEARCH_CHARS ? q : "",
    role: pickKnown(ROLES, params.get("role")),
    state: pickKnown(STATES, params.get("state")),
    page: parsePageParam(params.get("page")),
  };
}

/** Writes the criteria in a fixed order, leaving out empty values and page 1. */
export function toAdminUsersUrlParams(criteria: AdminUsersCriteria): URLSearchParams {
  const params = new URLSearchParams();
  if (criteria.q) params.set("q", criteria.q);
  if (criteria.role) params.set("role", criteria.role);
  if (criteria.state) params.set("state", criteria.state);
  if (criteria.page > 1) params.set("page", String(criteria.page));
  return params;
}

export function toAdminUsersRequest(criteria: AdminUsersCriteria): ListAdminUsersParams {
  return {
    q: criteria.q || undefined,
    role: criteria.role || undefined,
    state: criteria.state || undefined,
    page: criteria.page,
  };
}
