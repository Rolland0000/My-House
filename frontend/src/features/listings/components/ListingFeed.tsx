import { useEffect, useMemo, useState } from "react";
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
import { usePagination } from "../../../shared/hooks/usePagination";
import { useListings } from "../hooks/useListings";
import { typeLabels } from "../labels";
import { ListingCard } from "./ListingCard";
import type { ListingType } from "../api";

const CITY_FILTER_DEBOUNCE_MS = 400;
const GENERIC_ERROR_MESSAGE = "Please try again in a moment.";

const typeOptions = [
  { value: "", label: "All types" },
  ...Object.entries(typeLabels).map(([value, label]) => ({ value, label })),
];

function ListingFeed() {
  const [cityInput, setCityInput] = useState("");
  const [city, setCity] = useState("");
  const [type, setType] = useState<ListingType | "">("");

  useEffect(() => {
    const timeout = window.setTimeout(() => setCity(cityInput), CITY_FILTER_DEBOUNCE_MS);
    return () => window.clearTimeout(timeout);
  }, [cityInput]);

  const filters = useMemo(
    () => ({ city: city.trim() || undefined, type: type || undefined }),
    [city, type]
  );
  const { page, setPage } = usePagination(JSON.stringify(filters));

  const { data, isPending, isPlaceholderData, error } = useListings({ ...filters, page });

  const hasActiveFilters = Boolean(filters.city || filters.type);

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
        <Select
          value={type}
          onChange={(event) => setType(event.target.value as ListingType | "")}
          options={typeOptions}
          aria-label="Filter by property type"
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
          title={hasActiveFilters ? "No properties match these filters" : "No properties available right now"}
          description={
            hasActiveFilters ? "Widen the city or drop the type filter." : "Check back again soon."
          }
          secondaryAction={
            hasActiveFilters
              ? {
                  label: "Clear filters",
                  onClick: () => {
                    setCityInput("");
                    setType("");
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
