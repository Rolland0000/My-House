import { useEffect } from "react";
import { useQueryClient, type QueryClient } from "@tanstack/react-query";
import { useLocation, useNavigate } from "react-router";
import { removeAdminQueries } from "../../features/admin";
import { isSuspendedSignOut, SUSPENDED_SIGN_OUT_STATE, useAuth } from "../../features/auth";
import { removeContactQueries } from "../../features/contact";
import { removeOwnerScopedQueries } from "../../features/listings";
import { LOGIN_PATH } from "../../shared/utils/returnTo";

export function removeSessionQueries(queryClient: QueryClient) {
  removeOwnerScopedQueries(queryClient);
  removeContactQueries(queryClient);
  removeAdminQueries(queryClient);
}

/** Sends a suspended user to the sign-in page, once: afterwards they can browse as a visitor.
 *  `RequireAuth` may get there first without the state, so the flag is cleared only on arrival. */
export function useSuspendedRedirect() {
  const { wasSuspended, acknowledgeSuspension } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();
  const queryClient = useQueryClient();

  useEffect(() => {
    if (!wasSuspended) return;
    if (isSuspendedSignOut(location.state)) {
      acknowledgeSuspension();
      return;
    }
    removeSessionQueries(queryClient);
    navigate(LOGIN_PATH, { replace: true, state: SUSPENDED_SIGN_OUT_STATE });
  }, [wasSuspended, location.state, acknowledgeSuspension, navigate, queryClient]);
}
