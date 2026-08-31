import type { ReactNode } from "react";
import { Navigate } from "react-router";
import { useAuth } from "../features/auth";
import { Spinner } from "../shared/components";

interface RequireAuthProps {
  children: ReactNode;
}

/** Gates a route on an established session. The silent refresh runs first, so
 *  a page reload doesn't bounce a signed-in user to the login screen. */
export function RequireAuth({ children }: RequireAuthProps) {
  const { status } = useAuth();

  if (status === "bootstrapping") {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" />
      </div>
    );
  }

  if (status === "anonymous") {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}
