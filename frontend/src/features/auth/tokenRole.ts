import type { components } from "../../shared/api/types";

type Role = components["schemas"]["Role"];

const ROLES: readonly Role[] = ["seeker", "owner", "admin"];
const JWT_SEGMENT_COUNT = 3;

function decodeBase64Url(segment: string): string {
  const base64 = segment.replace(/-/g, "+").replace(/_/g, "/");
  return atob(base64.padEnd(Math.ceil(base64.length / 4) * 4, "="));
}

/** Reads the `role` claim without verifying the signature. For display only: the
 *  backend still decides what the token is allowed to do. */
export function readTokenRole(token: string): Role | null {
  const segments = token.split(".");
  if (segments.length !== JWT_SEGMENT_COUNT) return null;

  try {
    const payload: unknown = JSON.parse(decodeBase64Url(segments[1]));
    const role = (payload as { role?: unknown } | null)?.role;
    return ROLES.find((knownRole) => knownRole === role) ?? null;
  } catch {
    return null;
  }
}
