import { useMemo, useState } from "react";
import {
  Alert,
  DimensionRule,
  EmptyState,
  Input,
  Pagination,
  Select,
  Skeleton,
} from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { cn } from "../../../shared/utils/cn";
import { useDebouncedValue } from "../../../shared/hooks/useDebouncedValue";
import { usePagination } from "../../../shared/hooks/usePagination";
import { useListings } from "../hooks/useListings";
import { hasActiveFilters, toFeedFilters } from "../feedFilters";
import { AVAILABILITY_BADGE, typeLabels } from "../labels";
import { ListingCard } from "./ListingCard";
import type { ListingStatus, ListingType } from "../api";

const TEXT_FILTER_DEBOUNCE_MS = 400;
const GENERIC_ERROR_MESSAGE = "Please try again in a moment.";

const typeOptions = [
  { value: "", label: "All types" },
  ...Object.entries(typeLabels).map(([value, label]) => ({ value, label })),
];

const statusOptions = [
  { value: "", label: "All statuses" },
  ...Object.entries(AVAILABILITY_BADGE).map(([value, { label }]) => ({ value, label })),
];

function ListingFeed() {
  const [cityInput, setCityInput] = useState("");
  const [neighborhoodInput, setNeighborhoodInput] = useState("");
  const [type, setType] = useState<ListingType | "">("");
  const [status, setStatus] = useState<ListingStatus | "">("");

  // An emptied field applies at once, so "Clear filters" sends a single request.
  const city = useDebouncedValue(cityInput, cityInput ? TEXT_FILTER_DEBOUNCE_MS : 0);
  const neighborhood = useDebouncedValue(
    neighborhoodInput,
    neighborhoodInput ? TEXT_FILTER_DEBOUNCE_MS : 0
  );

  const filters = useMemo(
    () => toFeedFilters({ city, neighborhood, type, status }),
    [city, neighborhood, type, status]
  );
  const { page, setPage } = usePagination(JSON.stringify(filters));

  const { data, isPending, isPlaceholderData, error } = useListings({ ...filters, page });

  const filtersActive = hasActiveFilters(filters);

  return (
    <div className="mx-auto flex max-w-6xl flex-col gap-6 px-7 py-8">
      <div>
        <h1 className="text-2xl font-bold text-ink-900">Listings</h1>
        <DimensionRule width={120} className="mt-3.5" />
      </div>

      <div className="flex flex-wrap items-center gap-3">
        <Input
          value={cityInput}
          onChange={(event) => setCityInput(event.target.value)}
          placeholder="City"
          aria-label="Filter by city"
          className="max-w-xs"
        />
        <Input
          value={neighborhoodInput}
          onChange={(event) => setNeighborhoodInput(event.target.value)}
          placeholder="Neighborhood"
          aria-label="Filter by neighborhood"
          className="max-w-xs"
        />
        <Select
          value={type}
          onChange={(event) => setType(event.target.value as ListingType | "")}
          options={typeOptions}
          aria-label="Filter by property type"
          className="max-w-xs"
        />
        <Select
          value={status}
          onChange={(event) => setStatus(event.target.value as ListingStatus | "")}
          options={statusOptions}
          aria-label="Filter by status"
          className="max-w-xs"
        />
        <span className="flex-1" />
        {data && (
          <span className="text-sm font-medium text-ink-500">
            {data.pagination.total} propert{data.pagination.total === 1 ? "y" : "ies"} · page{" "}
            {data.pagination.page} of {data.pagination.total_pages}
          </span>
        )}
      </div>

      {error && (
        <Alert variant="error" title="Unable to load listings">
          {error instanceof ApiError ? GENERIC_ERROR_MESSAGE : GENERIC_ERROR_MESSAGE}
        </Alert>
      )}

      {isPending ? (
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 6 }).map((_, index) => (
            <Skeleton key={index} variant="card" />
          ))}
        </div>
      ) : data && data.data.length > 0 ? (
        <div
          className={cn(
            "grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3",
            isPlaceholderData && "opacity-60"
          )}
        >
          {data.data.map((listing) => (
            <ListingCard key={listing.id} listing={listing} />
          ))}
        </div>
      ) : data ? (
        <EmptyState
          title={
            filtersActive
              ? "No properties match these filters"
              : "No properties available right now"
          }
          description={
            filtersActive ? "Try fewer filters or check the spelling." : "Check back again soon."
          }
          secondaryAction={
            filtersActive
              ? {
                  label: "Clear filters",
                  onClick: () => {
                    setCityInput("");
                    setNeighborhoodInput("");
                    setType("");
                    setStatus("");
                  },
                }
              : undefined
          }
        />
      ) : null}

      {data && data.pagination.total_pages > 1 && (
        <Pagination
          page={data.pagination.page}
          totalPages={data.pagination.total_pages}
          onPageChange={setPage}
          className="self-center"
        />
      )}
    </div>
  );
}

export { ListingFeed };
