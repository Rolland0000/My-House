import { useQueryClient } from "@tanstack/react-query";
import { Outlet, useNavigate } from "react-router";
import { SiteHeader } from "../../shared/components";
import { logout, useAuth } from "../../features/auth";
import { removeAdminQueries, useOwnerRequestQueue } from "../../features/admin";
import { removeOwnerScopedQueries } from "../../features/listings";
import { useProfile } from "../../features/profile";

export function AdminLayout() {
  const { clearSession } = useAuth();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  // per_page: 1 only to read pagination.total for the header badge — a separate
  // cache entry from the queue list's own query, not meant to share one.
  const { data: profile } = useProfile();
  const { data } = useOwnerRequestQueue({ status: "pending", page: 1, per_page: 1 });

  async function handleSignOut() {
    try {
      await logout();
    } finally {
      clearSession();
      removeOwnerScopedQueries(queryClient);
      removeAdminQueries(queryClient);
      navigate("/");
    }
  }

  return (
    <div className="flex min-h-screen flex-col">
      <SiteHeader
        role="admin"
        adminEmail={profile?.email}
        pendingRequestCount={data?.pagination.total}
        onSignOut={handleSignOut}
      />
      <main className="flex-1 bg-paper-50">
        <Outlet />
      </main>
    </div>
  );
}
