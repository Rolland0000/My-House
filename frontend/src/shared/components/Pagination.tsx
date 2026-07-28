import { ChevronLeft, ChevronRight } from "lucide-react";
import { cn } from "../utils/cn";
import { getPageItems } from "../utils/pagination";

interface PaginationProps {
  page: number;
  totalPages: number;
  onPageChange: (page: number) => void;
  siblingCount?: number;
  className?: string;
}

const navButtonClasses =
  "inline-flex size-8 items-center justify-center rounded-sm text-text-muted hover:bg-primary-soft hover:text-text " +
  "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:bg-transparent";

function Pagination({
  page,
  totalPages,
  onPageChange,
  siblingCount = 1,
  className,
}: PaginationProps) {
  if (totalPages <= 1) return null;

  const items = getPageItems(page, totalPages, siblingCount);

  return (
    <nav aria-label="Pagination" className={cn("flex items-center gap-1", className)}>
      <button
        type="button"
        aria-label="Page précédente"
        className={navButtonClasses}
        disabled={page <= 1}
        onClick={() => onPageChange(page - 1)}
      >
        <ChevronLeft className="size-4" aria-hidden="true" />
      </button>

      {items.map((item) =>
        typeof item === "number" ? (
          <button
            key={item}
            type="button"
            aria-current={item === page ? "page" : undefined}
            className={cn(
              "inline-flex size-8 items-center justify-center rounded-sm text-sm font-semibold",
              "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus",
              item === page ? "bg-primary text-white" : "text-text hover:bg-primary-soft"
            )}
            onClick={() => onPageChange(item)}
          >
            {item}
          </button>
        ) : (
          <span
            key={item}
            aria-hidden="true"
            className="inline-flex size-8 items-center justify-center text-text-muted"
          >
            …
          </span>
        )
      )}

      <button
        type="button"
        aria-label="Page suivante"
        className={navButtonClasses}
        disabled={page >= totalPages}
        onClick={() => onPageChange(page + 1)}
      >
        <ChevronRight className="size-4" aria-hidden="true" />
      </button>
    </nav>
  );
}

export { Pagination };
export type { PaginationProps };
