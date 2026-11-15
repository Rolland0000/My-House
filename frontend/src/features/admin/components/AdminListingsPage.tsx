import { useEffect, useMemo, useRef } from "react";
import { useSearchParams } from "react-router";
import {
  Alert,
  Button,
  EmptyState,
  OwnerFilterChip,
  Pagination,
  Select,
  Spinner,
} from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";
import { ownerFilterLabel } from "../../../shared/utils/ownerFilter";
import { AVAILABILITY_BADGE, LISTING_STATUSES } from "../../listings";
import { ADMIN_VISIBILITY_BADGE } from "../adminListingVisibilityBadge";
import {
  hasAdminListingsFilters,
  LISTING_VISIBILITIES,
  parseAdminListingsCriteria,
  toAdminListingsRequest,
  toAdminListingsUrlParams,
  type AdminListingsCriteria,
} from "../adminListingsCriteria";
import { useAdminListings } from "../hooks/useAdminListings";
import { AdminListingRow } from "./AdminListingRow";
import { AdminSearchBox } from "./AdminSearchBox";

const STATUS_OPTIONS = [
  { value: "", label: "All statuses" },
  ...LISTING_STATUSES.map((status) => ({ value: status, label: AVAILABILITY_BADGE[status].label })),
];

const VISIBILITY_OPTIONS = [
  { value: "", label: "All visibilities" },
  ...LISTING_VISIBILITIES.map((visibility) => ({
    value: visibility,
    label: ADMIN_VISIBILITY_BADGE[visibility].label,
  })),
];

const COLUMNS = ["Title", "Owner", "City", "Type", "Status", "Visibility", "Created"];

function AdminListingsPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const criteria = useMemo(() => parseAdminListingsCriteria(searchParams), [searchParams]);
  const { data, isPending, isFetching, isPlaceholderData, error, refetch } = useAdminListings(
    toAdminListingsRequest(criteria)
  );
  const resultCountRef = useRef<HTMLParagraphElement>(null);
  const totalPages = data?.pagination.total_pages ?? 0;

  // A page past the end (e.g. after deletions) is swapped for the last one, without a history
  // entry. Placeholder data comes from other filters, so its page count doesn't apply.
  useEffect(() => {
    if (!isPlaceholderData && totalPages > 0 && criteria.page > totalPages) {
      setSearchParams(toAdminListingsUrlParams({ ...criteria, page: totalPages }), {
        replace: true,
      });
    }
  }, [criteria, isPlaceholderData, totalPages, setSearchParams]);

  function changeFilters(patch: Partial<AdminListingsCriteria>, replace = false) {
    setSearchParams(toAdminListingsUrlParams({ ...criteria, ...patch, page: 1 }), { replace });
  }

  function removeOwnerFilter() {
    changeFilters({ ownerId: "" });
    // The chip's button is about to unmount, which would drop keyboard focus to <body>.
    resultCountRef.current?.focus();
  }

  return (
    <div className="mx-auto flex w-full max-w-6xl flex-col gap-6 px-4 py-10">
      <h1 className="text-2xl font-bold text-ink-900">Properties</h1>

      <div className="flex flex-wrap items-center gap-3">
        <AdminSearchBox
          target="title, city or owner"
          value={criteria.q}
          onSearch={(q, settled) => changeFilters({ q }, settled)}
        />
        <div className="flex flex-wrap items-center gap-3">
          <Select
            aria-label="Filter by status"
            className="w-44"
            value={criteria.status}
            options={STATUS_OPTIONS}
            onChange={(event) =>
              changeFilters({ status: event.target.value as AdminListingsCriteria["status"] })
            }
          />
          <Select
            aria-label="Filter by visibility"
            className="w-44"
            value={criteria.visibility}
            options={VISIBILITY_OPTIONS}
            onChange={(event) =>
              changeFilters({
                visibility: event.target.value as AdminListingsCriteria["visibility"],
              })
            }
          />
        </div>
      </div>

      {criteria.ownerId && (
        <OwnerFilterChip
          label={ownerFilterLabel(criteria.ownerId, data?.data ?? [])}
          onRemove={removeOwnerFilter}
        />
      )}

      <div className="flex min-h-5 items-center gap-2 text-sm font-medium text-ink-500">
        <p ref={resultCountRef} tabIndex={-1} aria-live="polite">
          {data && `${data.pagination.total} propert${data.pagination.total === 1 ? "y" : "ies"}`}
        </p>
        {isPlaceholderData && <Spinner size="sm" label="Updating results…" />}
      </div>

      {error && (
        <Alert variant="error" title="Unable to load properties">
          <div className="flex flex-wrap items-center gap-3">
            <span>Please try again in a moment.</span>
            <Button variant="secondary" size="sm" isLoading={isFetching} onClick={() => refetch()}>
              Try again
            </Button>
          </div>
        </Alert>
      )}

      {isPending ? (
        <div className="flex justify-center py-16">
          <Spinner size="lg" label="Loading…" />
        </div>
      ) : data?.data.length ? (
        <>
          {/* Focusable so keyboard users can scroll the table sideways at tablet width.
              `relative` makes it clip the sr-only elements, which would otherwise widen the page. */}
          <div
            role="region"
            aria-label="Properties table"
            aria-busy={isPlaceholderData}
            tabIndex={0}
            className={cn(
              "relative overflow-x-auto border border-border bg-surface focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus",
              isPlaceholderData && "opacity-60"
            )}
          >
            <table className="w-full min-w-5xl text-left text-sm">
              <caption className="sr-only">Every property on the platform</caption>
              <thead>
                <tr className="border-b border-border text-xs font-semibold text-text-muted">
                  {COLUMNS.map((column) => (
                    <th key={column} scope="col" className="px-4 py-3">
                      {column}
                    </th>
                  ))}
                  <th scope="col" className="px-4 py-3">
                    <span className="sr-only">Actions</span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {data.data.map((listing) => (
                  <AdminListingRow key={listing.id} listing={listing} />
                ))}
              </tbody>
            </table>
          </div>

          <Pagination
            page={criteria.page}
            totalPages={totalPages}
            onPageChange={(page) =>
              setSearchParams(toAdminListingsUrlParams({ ...criteria, page }))
            }
            className="justify-center"
          />
        </>
      ) : data?.pagination.total === 0 ? (
        hasAdminListingsFilters(criteria) ? (
          <EmptyState
            title="No properties match these filters"
            primaryAction={{ label: "Clear filters", onClick: () => setSearchParams({}) }}
          />
        ) : (
          <EmptyState
            title="No properties yet"
            description="Properties appear here as soon as an owner creates one."
          />
        )
      ) : null}
    </div>
  );
}

export { AdminListingsPage };
