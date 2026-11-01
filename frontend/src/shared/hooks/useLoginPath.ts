import { createPath, useLocation } from "react-router";
import { loginPath } from "../utils/returnTo";

/** The sign-in page, set to bring the user back to the current page. */
export function useLoginPath(): string {
  return loginPath(createPath(useLocation()));
}
