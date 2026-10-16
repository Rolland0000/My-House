import { useId, useMemo, useState } from "react";
import { useSearchParams } from "react-router";
import { ChevronDown } from "lucide-react";
import {
  Alert,
  Button,
  DimensionRule,
  EmptyState,
  Pagination,
  Skeleton,
  Spinner,
} from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";
import { ListingCard } from "../../listings";
import { useSearch } from "../hooks/useSearch";
import {
  EMPTY_CRITERIA,
  hasSearchCriteria,
  parseSearchCriteria,
  toSearchRequest,
  toUrlSearchParams,
  type SearchCriteria,
} from "../searchCriteria";
import { FilterPanel, type FilterPatch } from "./FilterPanel";
import { SearchBar } from "./SearchBar";

const SKELETON_CARD_COUNT = 6;
const RESULTS_GRID_CLASSES = "grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-3";

function SearchPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const criteria = useMemo(() => parseSearchCriteria(searchParams), [searchParams]);
  const request = useMemo(() => toSearchRequest(criteria), [criteria]);
  const { data, isPending, isFetching, isPlaceholderData, error, refetch } = useSearch(request);

  const filtersPanelId = useId();
  const [isFiltersOpen, setIsFiltersOpen] = useState(false);
  // Bumped by "Clear all" to remount the text fields and drop their unsent drafts.
  const [draftResetKey, setDraftResetKey] = useState(0);

  function navigate(next: SearchCriteria, replace = false) {
    const nextParams = toUrlSearchParams(next);
    if (nextParams.toString() === searchParams.toString()) return;
    // flushSync renders the new URL at once. Until it renders, a ticked box shows its old
    // state and a field resynced from the URL could drop text typed meanwhile.
    setSearchParams(nextParams, { replace, flushSync: true });
  }

  function changeFilters(patch: FilterPatch) {
    navigate({ ...criteria, ...patch, page: 1 }, true);
  }

  function submitKeyword(keyword: string) {
    navigate({ ...criteria, q: keyword, page: 1 });
  }

  function clearAll() {
    setDraftResetKey((key) => key + 1);
    navigate(EMPTY_CRITERIA, true);
  }

  const hasCriteria = hasSearchCriteria(criteria);

  return (
    <div className="mx-auto flex max-w-6xl flex-col gap-6 px-7 py-8">
      <div>
        <h1 className="text-2xl font-bold text-ink-900">Search</h1>
        <DimensionRule width={120} className="mt-3.5" />
      </div>

      <div className="flex flex-col gap-6 md:flex-row md:items-start">
        <aside aria-label="Filters" className="md:w-60 md:flex-none">
          <button
            type="button"
            aria-expanded={isFiltersOpen}
            aria-controls={filtersPanelId}
            onClick={() => setIsFiltersOpen((open) => !open)}
            className="flex w-full items-center justify-between rounded-sm border border-border bg-surface px-4 py-3 text-sm font-semibold text-text focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus md:hidden"
          >
            Filters
            <ChevronDown
              aria-hidden="true"
              className={cn("size-4 text-ink-500", isFiltersOpen && "rotate-180")}
            />
          </button>
          <div
            id={filtersPanelId}
            className={cn("mt-3 md:mt-0 md:block", !isFiltersOpen && "hidden")}
          >
            <FilterPanel
              criteria={criteria}
              draftResetKey={draftResetKey}
              onChange={changeFilters}
              onClearAll={clearAll}
            />
          </div>
        </aside>

        <div className="flex min-w-0 flex-1 flex-col gap-6">
          <SearchBar key={draftResetKey} value={criteria.q} onSubmit={submitKeyword} />

          <div className="flex min-h-5 items-center gap-2 text-sm font-medium text-ink-500">
            <p aria-live="polite">
              {data &&
                `${data.pagination.total} propert${data.pagination.total === 1 ? "y" : "ies"}`}
            </p>
            {isPlaceholderData && <Spinner size="sm" label="Updating results…" />}
          </div>

          {error && (
            <Alert variant="error" title="Unable to load results">
              <div className="flex flex-wrap items-center gap-3">
                <span>Please try again in a moment.</span>
                <Button
                  variant="secondary"
                  size="sm"
                  isLoading={isFetching}
                  onClick={() => refetch()}
                >
                  Try again
                </Button>
              </div>
            </Alert>
          )}

          {isPending ? (
            <div className={RESULTS_GRID_CLASSES}>
              {Array.from({ length: SKELETON_CARD_COUNT }).map((_, index) => (
                <Skeleton key={index} variant="card" />
              ))}
            </div>
          ) : data && data.data.length > 0 ? (
            <div
              aria-busy={isPlaceholderData}
              className={cn(RESULTS_GRID_CLASSES, isPlaceholderData && "opacity-60")}
            >
              {data.data.map((listing) => (
                <ListingCard key={listing.id} listing={listing} />
              ))}
            </div>
          ) : data && data.pagination.total > 0 ? (
            // A page past the last one, e.g. from an old shared link.
            <EmptyState
              title="No properties on this page"
              secondaryAction={{
                label: "Go to page 1",
                onClick: () => navigate({ ...criteria, page: 1 }),
              }}
            />
          ) : data ? (
            <EmptyState
              title={
                hasCriteria
                  ? "No properties match your search"
                  : "No properties available right now"
              }
              description={
                hasCriteria ? "Try other words or fewer filters." : "Check back again soon."
              }
              secondaryAction={hasCriteria ? { label: "Clear all", onClick: clearAll } : undefined}
            />
          ) : null}

          {data && data.pagination.total_pages > 1 && (
            <Pagination
              page={data.pagination.page}
              totalPages={data.pagination.total_pages}
              onPageChange={(page) => navigate({ ...criteria, page })}
              className="self-center"
            />
          )}
        </div>
      </div>
    </div>
  );
}

export { SearchPage };
