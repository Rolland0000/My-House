import { createPath, matchPath } from "react-router";

export const LOGIN_PATH = "/login";
export const ADMIN_LOGIN_PATH = "/admin/login";
const ADMIN_HOME_PATH = "/admin";
export const RETURN_TO_PARAM = "redirect";
const HOME_PATH = "/";
// Any origin works: parsing against it only tells whether the target stays on the same one.
const PARSE_BASE = "http://return-to.invalid";

/** The sign-in page, set to bring the user back to `path` afterwards. Home needs no target. */
export function loginPath(path: string): string {
  if (path === HOME_PATH) return LOGIN_PATH;
  return `${LOGIN_PATH}?${new URLSearchParams({ [RETURN_TO_PARAM]: path })}`;
}

/**
 * The in-app path to go to after signing in, or home when `raw` is missing or unsafe.
 * Parsing catches what a prefix check misses: `//host`, `/\host` and tabs or newlines
 * that browsers strip all resolve to another origin.
 */
export function safeReturnTo(raw: string | null): string {
  if (!raw?.startsWith("/")) return HOME_PATH;

  let url: URL;
  try {
    url = new URL(raw, PARSE_BASE);
  } catch {
    // An invalid host such as `//[` throws, and this runs during render.
    return HOME_PATH;
  }
  if (url.origin !== PARSE_BASE) return HOME_PATH;

  // Matched like the route itself, so `/LOGIN/` counts as the sign-in page too.
  if (matchPath(LOGIN_PATH, url.pathname)) return HOME_PATH;

  return createPath(url);
}

/** The admin sign-in page, set to bring the admin back to `path` afterwards. */
export function adminLoginPath(path: string): string {
  if (path === ADMIN_HOME_PATH) return ADMIN_LOGIN_PATH;
  return `${ADMIN_LOGIN_PATH}?${new URLSearchParams({ [RETURN_TO_PARAM]: path })}`;
}

/** Like [`safeReturnTo`], but only accepts a page of the admin area; anything else lands on its home. */
export function safeAdminReturnTo(raw: string | null): string {
  const target = safeReturnTo(raw);
  const isAdminArea = target === ADMIN_HOME_PATH || target.startsWith(`${ADMIN_HOME_PATH}/`);
  if (!isAdminArea || matchPath(ADMIN_LOGIN_PATH, new URL(target, PARSE_BASE).pathname)) {
    return ADMIN_HOME_PATH;
  }
  return target;
}
