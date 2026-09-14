import type { ReactNode } from "react";
import { Navigate } from "react-router";
import { useProfile } from "../features/profile";
import { Spinner } from "../shared/components";

interface RequireAdminProps {
  children: ReactNode;
}

/** Nested inside `RequireAuth` — assumes the session is already established
 *  and only adds the role check on top. */
export function RequireAdmin({ children }: RequireAdminProps) {
  const { data: profile, isPending } = useProfile();

  if (isPending) {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" />
      </div>
    );
  }

  if (profile?.role !== "admin") {
    return <Navigate to="/" replace />;
  }

  return <>{children}</>;
}
