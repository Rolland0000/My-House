import { Link } from "react-router";
import { PRICE_CURRENCY } from "../utils/format";

type SiteFooterVariant = "public" | "none";

interface SiteFooterProps {
  variant?: SiteFooterVariant;
}

interface FooterLink {
  label: string;
  /** Left out while the page doesn't exist: the label then renders as plain text, never a dead link. */
  to?: string;
}

const columns: { heading: string; links: FooterLink[] }[] = [
  {
    heading: "Explore",
    links: [
      { label: "Home", to: "/" },
      { label: "Listings", to: "/" },
      { label: "Search", to: "/search" },
    ],
  },
  {
    heading: "Company",
    links: [
      { label: "About", to: "/about" },
      { label: "Contact", to: "/contact" },
    ],
  },
  {
    heading: "Owners",
    links: [
      { label: "Post a property", to: "/owner/listings/new" },
      { label: "Become a verified owner", to: "/owner-request" },
    ],
  },
  {
    heading: "Legal",
    links: [{ label: "Legal notice" }, { label: "Terms of use" }, { label: "Report a listing" }],
  },
];

const FOOTER_LINK_CLASS = "text-sm font-medium text-[#DDE2E9]";

function SiteFooter({ variant = "public" }: SiteFooterProps) {
  if (variant === "none") return null;

  return (
    <footer className="bg-ink-900 px-7 pt-9 pb-7">
      <div className="mx-auto flex max-w-[1112px] flex-col gap-7">
        <div className="flex flex-wrap items-start justify-between gap-12">
          <div className="max-w-[280px]">
            <div className="mb-2 flex items-baseline gap-1.5">
              <span className="text-base font-extrabold tracking-tight text-white">MY HOUSE</span>
              <span className="size-[5px] bg-primary" aria-hidden="true" />
            </div>
            <p className="text-sm text-[#B8C0CE]">
              Direct contact between owners and renters, across France and French-speaking Africa.
            </p>
          </div>

          <div className="flex flex-wrap gap-12">
            {columns.map((column) => (
              <div key={column.heading}>
                <p className="mb-3 text-xs font-semibold text-white">{column.heading}</p>
                <div className="flex flex-col gap-2">
                  {column.links.map((link) =>
                    link.to ? (
                      <Link
                        key={link.label}
                        to={link.to}
                        className={`${FOOTER_LINK_CLASS} hover:text-white hover:underline`}
                      >
                        {link.label}
                      </Link>
                    ) : (
                      <span key={link.label} className={FOOTER_LINK_CLASS}>
                        {link.label}
                      </span>
                    )
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="h-px bg-white/10" />
        <p className="text-xs text-[#8E9AAC]">© 2026 My House. All prices in {PRICE_CURRENCY}.</p>
      </div>
    </footer>
  );
}

export { SiteFooter };
export type { SiteFooterProps, SiteFooterVariant };
