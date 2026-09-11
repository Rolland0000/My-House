type SiteFooterVariant = "public" | "none";

interface SiteFooterProps {
  variant?: SiteFooterVariant;
}

const columns: { heading: string; links: string[] }[] = [
  { heading: "Explore", links: ["Home", "Listings", "Search"] },
  { heading: "Company", links: ["About", "Contact"] },
  { heading: "Owners", links: ["Post a property", "Become a verified owner"] },
  { heading: "Legal", links: ["Legal notice", "Terms of use", "Report a listing"] },
];

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
                  {column.links.map((link) => (
                    <span key={link} className="text-sm font-medium text-[#DDE2E9]">
                      {link}
                    </span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="h-px bg-white/10" />
        <p className="text-xs text-[#8E9AAC]">© 2026 My House. All prices in FCFA.</p>
      </div>
    </footer>
  );
}

export { SiteFooter };
export type { SiteFooterProps, SiteFooterVariant };
