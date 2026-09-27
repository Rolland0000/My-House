import { useQueryClient } from "@tanstack/react-query";
import { Outlet, useNavigate } from "react-router";
import { SiteFooter, SiteHeader } from "../../shared/components";
import type { SiteHeaderRole } from "../../shared/components";
import { logout, useAuth } from "../../features/auth";
import { removeOwnerScopedQueries } from "../../features/listings";
import { ownerRequestStatusView, useOwnerRequestStatus } from "../../features/owner-request";
import { useProfile } from "../../features/profile";
import { formatInitials } from "../../shared/utils/format";

export function RootLayout() {
  const { status, clearSession } = useAuth();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { data: profile } = useProfile({ enabled: status === "authenticated" });

  const role: SiteHeaderRole =
    status === "authenticated" && profile?.role === "owner"
      ? "owner"
      : status === "authenticated"
        ? "seeker"
        : "public";

  // Admins also browse with the seeker header, but can't apply — only real seekers get the CTA.
  const isSeeker = status === "authenticated" && profile?.role === "seeker";
  const { data: ownerRequest } = useOwnerRequestStatus({ enabled: isSeeker });
  const ownerRequestView =
    isSeeker && ownerRequest !== undefined ? ownerRequestStatusView(ownerRequest) : null;

  const user =
    status === "authenticated" && profile
      ? {
          initials: formatInitials(profile.first_name, profile.last_name) || "?",
          name: [profile.first_name, profile.last_name].filter(Boolean).join(" ") || profile.email,
        }
      : null;

  async function handleSignOut() {
    try {
      await logout();
    } finally {
      clearSession();
      removeOwnerScopedQueries(queryClient);
      navigate("/");
    }
  }

  return (
    <div className="flex min-h-screen flex-col">
      <SiteHeader
        role={role}
        user={user}
        ownerRequestStatus={ownerRequestView?.state === "pending" ? "pending" : null}
        ownerRequestCta={ownerRequestView?.showCta ?? false}
        onSignOut={handleSignOut}
      />
      <main className="flex-1">
        <Outlet />
      </main>
      <SiteFooter variant="public" />
    </div>
  );
}
