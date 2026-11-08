/** Location state of the redirect to the sign-in page, read by `AuthFlow`. */
export const SUSPENDED_SIGN_OUT_STATE = { accountSuspended: true };

export function isSuspendedSignOut(locationState: unknown): boolean {
  return (
    typeof locationState === "object" &&
    locationState !== null &&
    "accountSuspended" in locationState
  );
}
