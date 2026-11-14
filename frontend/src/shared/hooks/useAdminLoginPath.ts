import { createPath, useLocation } from "react-router";
import { adminLoginPath } from "../utils/returnTo";

/** The admin sign-in page, set to bring the admin back to the current page. */
export function useAdminLoginPath(): string {
  return adminLoginPath(createPath(useLocation()));
}
