import type { ReactNode } from "react";
import { Navigate } from "react-router";
import { useAuth } from "../features/auth";
import { Spinner } from "../shared/components";
import { useLoginPath } from "../shared/hooks/useLoginPath";
import { useAdminLoginPath } from "../shared/hooks/useAdminLoginPath";

interface RequireAuthProps {
  children: ReactNode;
  /** Sends an anonymous visitor to the admin sign-in page instead of the public one. */
  adminArea?: boolean;
}

/** Gates a route on an established session. The silent refresh runs first, so
 *  a page reload doesn't bounce a signed-in user to the login screen. */
export function RequireAuth({ children, adminArea = false }: RequireAuthProps) {
  const { status } = useAuth();
  const publicSignInPath = useLoginPath();
  const adminSignInPath = useAdminLoginPath();
  const signInPath = adminArea ? adminSignInPath : publicSignInPath;

  if (status === "bootstrapping") {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" />
      </div>
    );
  }

  if (status === "anonymous") {
    return <Navigate to={signInPath} replace />;
  }

  return <>{children}</>;
}
