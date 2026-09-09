import { useEffect, useRef, useState, type ReactNode } from "react";
import { Link } from "react-router";
import { ChevronDown, Menu, X } from "lucide-react";
import { Badge } from "./Badge";
import { cn } from "../utils/cn";

type SiteHeaderRole = "public" | "seeker" | "owner" | "admin";

interface SiteHeaderUser {
  initials: string;
  name: string;
}

interface SiteHeaderProps {
  role: SiteHeaderRole;
  user?: SiteHeaderUser | null;
  /** Seeker only — no data source yet, accepted for a future pending-request badge. */
  ownerRequestStatus?: "pending" | null;
  /** Owner only — no data source yet, accepted for a future verified-owner chip. */
  verified?: boolean;
  /** Admin only — no admin pages are mounted yet, accepted for a future queue count. */
  pendingRequestCount?: number;
  onSignOut?: () => void;
}

interface NavLinkSpec {
  label: string;
  to?: string;
  current?: boolean;
}

const wordmark = (
  <Link to="/" className="flex flex-none items-baseline gap-1.5">
    <span className="text-[19px] font-extrabold tracking-tight text-ink-900">MY HOUSE</span>
    <span className="size-1.5 bg-primary" aria-hidden="true" />
  </Link>
);

/** A nav item with no real route yet renders as inert text, matching the brief's
 *  "bare, unlinked" treatment for About/Contact and any owner/admin page that
 *  doesn't exist in the router yet — never a dead link. */
function NavItem({ to, current, children }: { to?: string; current?: boolean; children: string }) {
  const classes = cn(
    "border-b-2 py-1.5 text-sm",
    current ? "border-ink-900 font-semibold text-ink-900" : "border-transparent font-medium text-ink-500"
  );

  if (!to) {
    return <span className={cn(classes, "cursor-default")}>{children}</span>;
  }
  return (
    <Link to={to} className={classes}>
      {children}
    </Link>
  );
}

/** Below `sm`, the nav/actions row would overflow — collapse it into a hamburger
 *  toggle instead (matches the mockup's mobile header, id `1c`). */
function MobileMenu({ links, onClose, children }: { links: NavLinkSpec[]; onClose: () => void; children?: ReactNode }) {
  return (
    <div className="border-t border-border bg-surface px-7 py-4 sm:hidden">
      <nav className="flex flex-col">
        {links.map((link) =>
          link.to ? (
            <Link
              key={link.label}
              to={link.to}
              onClick={onClose}
              className={cn(
                "border-b border-[#E7E8E4] py-3 text-sm",
                link.current ? "font-semibold text-ink-900" : "font-medium text-ink-500"
              )}
            >
              {link.label}
            </Link>
          ) : (
            <span key={link.label} className="border-b border-[#E7E8E4] py-3 text-sm font-medium text-ink-500">
              {link.label}
            </span>
          )
        )}
      </nav>
      {children && <div className="flex flex-col gap-2.5 pt-3.5">{children}</div>}
    </div>
  );
}

function UserMenu({ user, profileHref, onSignOut }: { user: SiteHeaderUser; profileHref: string; onSignOut?: () => void }) {
  const [isOpen, setIsOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isOpen) return;

    function handleClickOutside(event: MouseEvent) {
      if (!containerRef.current?.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }
    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") setIsOpen(false);
    }

    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isOpen]);

  return (
    <div ref={containerRef} className="relative flex flex-none items-center gap-2.5 border-l border-[#E7E8E4] pl-4">
      <button
        type="button"
        onClick={() => setIsOpen((open) => !open)}
        aria-expanded={isOpen}
        className="flex items-center gap-2.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
      >
        <span className="flex size-8 items-center justify-center bg-ink-900 text-sm font-semibold text-white">
          {user.initials}
        </span>
        <span className="hidden text-sm font-semibold text-ink-900 md:inline">{user.name}</span>
        <ChevronDown className="size-3.5 text-ink-500" aria-hidden="true" />
      </button>

      {isOpen && (
        <div className="absolute top-full right-0 z-10 mt-2 w-55 rounded-lg border border-border bg-surface py-1.5 shadow-elevated">
          <Link
            to={profileHref}
            className="block px-3.5 py-2 text-sm text-text hover:bg-primary-soft"
            onClick={() => setIsOpen(false)}
          >
            My profile
          </Link>
          <div className="my-1.5 h-px bg-[#E7E8E4]" />
          <button
            type="button"
            onClick={() => {
              setIsOpen(false);
              onSignOut?.();
            }}
            className="block w-full px-3.5 py-2 text-left text-sm text-error-text hover:bg-primary-soft"
          >
            Sign out
          </button>
        </div>
      )}
    </div>
  );
}

function HamburgerButton({ isOpen, onClick }: { isOpen: boolean; onClick: () => void }) {
  return (
    <button
      type="button"
      onClick={onClick}
      aria-label={isOpen ? "Close menu" : "Open menu"}
      aria-expanded={isOpen}
      className="flex size-11 flex-none items-center justify-center text-ink-900 sm:hidden"
    >
      {isOpen ? <X className="size-5" aria-hidden="true" /> : <Menu className="size-5" aria-hidden="true" />}
    </button>
  );
}

const PUBLIC_LINKS: NavLinkSpec[] = [
  { label: "Home", to: "/" },
  { label: "Listings", to: "/", current: true },
  { label: "Search" },
  { label: "About" },
  { label: "Contact" },
];

function PublicHeader() {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <>
      <div className="mx-auto flex h-[66px] max-w-[1112px] items-center gap-7 px-4 sm:px-7">
        {wordmark}
        <nav className="hidden flex-1 items-center gap-5 sm:flex">
          {PUBLIC_LINKS.map((link) =>
            link.to ? (
              <NavItem key={link.label} to={link.to} current={link.current}>
                {link.label}
              </NavItem>
            ) : (
              <NavItem key={link.label}>{link.label}</NavItem>
            )
          )}
        </nav>
        <div className="hidden flex-1 items-center justify-end gap-3 sm:flex">
          <Link to="/login" className="px-1 py-2.5 text-sm font-medium text-ink-600">
            Log in
          </Link>
          <Link
            to="/login"
            className="rounded-sm border border-brass-600 bg-primary px-4.5 py-3 text-sm font-semibold text-ink-900"
          >
            Post a property
          </Link>
        </div>
        <span className="flex-1 sm:hidden" />
        <HamburgerButton isOpen={isMenuOpen} onClick={() => setIsMenuOpen((open) => !open)} />
      </div>
      {isMenuOpen && (
        <MobileMenu links={PUBLIC_LINKS} onClose={() => setIsMenuOpen(false)}>
          <Link
            to="/login"
            onClick={() => setIsMenuOpen(false)}
            className="py-2 text-sm font-medium text-ink-600"
          >
            Log in
          </Link>
          <Link
            to="/login"
            onClick={() => setIsMenuOpen(false)}
            className="rounded-sm border border-brass-600 bg-primary px-4 py-3 text-center text-sm font-semibold text-ink-900"
          >
            Post a property
          </Link>
        </MobileMenu>
      )}
    </>
  );
}

function SeekerHeader({
  user,
  ownerRequestStatus,
  onSignOut,
}: {
  user?: SiteHeaderUser | null;
  ownerRequestStatus?: "pending" | null;
  onSignOut?: () => void;
}) {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <>
      <div className="mx-auto flex h-[66px] max-w-[1112px] items-center gap-7 px-4 sm:px-7">
        {wordmark}
        <nav className="hidden flex-1 items-center gap-5 sm:flex">
          {PUBLIC_LINKS.map((link) =>
            link.to ? (
              <NavItem key={link.label} to={link.to} current={link.current}>
                {link.label}
              </NavItem>
            ) : (
              <NavItem key={link.label}>{link.label}</NavItem>
            )
          )}
        </nav>
        <span className="flex-1 sm:hidden" />
        {ownerRequestStatus === "pending" && (
          <Badge tone="warning" className="hidden flex-none sm:inline-flex">
            Owner request pending
          </Badge>
        )}
        {user && (
          <div className="hidden sm:block">
            <UserMenu user={user} profileHref="/profile" onSignOut={onSignOut} />
          </div>
        )}
        <HamburgerButton isOpen={isMenuOpen} onClick={() => setIsMenuOpen((open) => !open)} />
      </div>
      {isMenuOpen && (
        <MobileMenu links={PUBLIC_LINKS} onClose={() => setIsMenuOpen(false)}>
          {ownerRequestStatus === "pending" && <Badge tone="warning">Owner request pending</Badge>}
          {user && (
            <>
              <Link to="/profile" onClick={() => setIsMenuOpen(false)} className="py-2 text-sm text-text">
                My profile
              </Link>
              <button
                type="button"
                onClick={() => {
                  setIsMenuOpen(false);
                  onSignOut?.();
                }}
                className="py-2 text-left text-sm text-error-text"
              >
                Sign out
              </button>
            </>
          )}
        </MobileMenu>
      )}
    </>
  );
}

const OWNER_LINKS: NavLinkSpec[] = [
  { label: "Listings", to: "/", current: true },
  { label: "My properties" },
  { label: "About" },
  { label: "Contact" },
];

function OwnerHeader({
  user,
  verified,
  onSignOut,
}: {
  user?: SiteHeaderUser | null;
  verified?: boolean;
  onSignOut?: () => void;
}) {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <>
      <div className="mx-auto flex h-[66px] max-w-[1112px] items-center gap-6 px-4 sm:px-7">
        {wordmark}
        <nav className="hidden flex-1 items-center gap-5 sm:flex">
          {OWNER_LINKS.map((link) =>
            link.to ? (
              <NavItem key={link.label} to={link.to} current={link.current}>
                {link.label}
              </NavItem>
            ) : (
              <NavItem key={link.label}>{link.label}</NavItem>
            )
          )}
        </nav>
        <span className="flex-1 sm:hidden" />
        {verified && (
          <Badge tone="brass" className="hidden flex-none sm:inline-flex">
            Verified owner
          </Badge>
        )}
        <span className="hidden flex-none cursor-default rounded-sm border border-brass-600 bg-primary px-4 py-3 text-sm font-semibold text-ink-900 sm:inline-flex">
          Add a property
        </span>
        {user && (
          <div className="hidden sm:block">
            <UserMenu user={user} profileHref="/profile" onSignOut={onSignOut} />
          </div>
        )}
        <HamburgerButton isOpen={isMenuOpen} onClick={() => setIsMenuOpen((open) => !open)} />
      </div>
      {isMenuOpen && (
        <MobileMenu links={OWNER_LINKS} onClose={() => setIsMenuOpen(false)}>
          {verified && <Badge tone="brass">Verified owner</Badge>}
          <span className="cursor-default rounded-sm border border-brass-600 bg-primary px-4 py-3 text-center text-sm font-semibold text-ink-900">
            Add a property
          </span>
          {user && (
            <>
              <Link to="/profile" onClick={() => setIsMenuOpen(false)} className="py-2 text-sm text-text">
                My profile
              </Link>
              <button
                type="button"
                onClick={() => {
                  setIsMenuOpen(false);
                  onSignOut?.();
                }}
                className="py-2 text-left text-sm text-error-text"
              >
                Sign out
              </button>
            </>
          )}
        </MobileMenu>
      )}
    </>
  );
}

function AdminHeader({ pendingRequestCount, onSignOut }: { pendingRequestCount?: number; onSignOut?: () => void }) {
  return (
    <div className="flex h-13 items-center gap-5.5 px-5.5">
      <span className="flex flex-none items-baseline gap-1.5">
        <span className="text-[15px] font-extrabold tracking-tight text-white">MY HOUSE</span>
        <span className="text-[11px] font-semibold tracking-[.06em] text-primary">ADMIN</span>
      </span>
      <nav className="hidden flex-1 items-center gap-0.5 sm:flex">
        <span className="flex cursor-default items-center gap-1 bg-paper-50 px-3.5 py-2 text-sm font-semibold text-ink-900">
          Owner requests
          {Boolean(pendingRequestCount) && (
            <span className="bg-warning-soft px-1.5 py-0.5 text-xs font-bold text-warning-text">
              {pendingRequestCount}
            </span>
          )}
        </span>
        <span className="cursor-default px-3.5 py-2 text-sm text-[#B8C0CE]">Users</span>
        <span className="cursor-default px-3.5 py-2 text-sm text-[#B8C0CE]">Properties</span>
      </nav>
      <span className="flex-1 sm:hidden" />
      <span className="hidden flex-none text-sm text-[#8E9AAC] sm:inline">admin@myhouse.app</span>
      <button
        type="button"
        onClick={onSignOut}
        className="flex-none text-sm text-[#DDE2E9] hover:text-white"
      >
        Sign out
      </button>
    </div>
  );
}

function SiteHeader({ role, user, ownerRequestStatus, verified, pendingRequestCount, onSignOut }: SiteHeaderProps) {
  if (role === "admin") {
    return (
      <header className="bg-ink-900">
        <AdminHeader pendingRequestCount={pendingRequestCount} onSignOut={onSignOut} />
      </header>
    );
  }

  return (
    <header className="border-b border-border bg-surface">
      {role === "public" && <PublicHeader />}
      {role === "seeker" && (
        <SeekerHeader user={user} ownerRequestStatus={ownerRequestStatus} onSignOut={onSignOut} />
      )}
      {role === "owner" && <OwnerHeader user={user} verified={verified} onSignOut={onSignOut} />}
    </header>
  );
}

export { SiteHeader };
export type { SiteHeaderProps, SiteHeaderRole, SiteHeaderUser };
