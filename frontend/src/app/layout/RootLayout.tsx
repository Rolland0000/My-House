import { Outlet, useNavigate } from "react-router";
import { SiteFooter, SiteHeader } from "../../shared/components";
import type { SiteHeaderRole } from "../../shared/components";
import { logout, useAuth } from "../../features/auth";
import { useProfile } from "../../features/profile";
import { formatInitials } from "../../shared/utils/format";

export function RootLayout() {
  const { status, clearSession } = useAuth();
  const navigate = useNavigate();
  const { data: profile } = useProfile({ enabled: status === "authenticated" });

  const role: SiteHeaderRole =
    status === "authenticated" && profile?.role === "owner" ? "owner" : status === "authenticated" ? "seeker" : "public";

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
      navigate("/");
    }
  }

  return (
    <div className="flex min-h-screen flex-col">
      <SiteHeader role={role} user={user} onSignOut={handleSignOut} />
      <main className="flex-1">
        <Outlet />
      </main>
      <SiteFooter variant="public" />
    </div>
  );
}
