import { useEffect, useMemo, useState } from "react";
import { Alert, Input, Pagination, Select, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { cn } from "../../../shared/utils/cn";
import { usePagination } from "../../../shared/hooks/usePagination";
import { useListings } from "../hooks/useListings";
import { typeLabels } from "../labels";
import { ListingCard } from "./ListingCard";
import type { ListingType } from "../api";

const CITY_FILTER_DEBOUNCE_MS = 400;

const typeOptions = [
  { value: "", label: "Tous les types" },
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
    <div className="mx-auto flex max-w-6xl flex-col gap-6 p-6">
      <h1 className="text-lg font-bold text-text">Découvrez des logements</h1>

      <div className="flex flex-wrap gap-3">
        <Input
          value={cityInput}
          onChange={(event) => setCityInput(event.target.value)}
          placeholder="Ville"
          aria-label="Filtrer par ville"
          className="max-w-xs"
        />
        <Select
          value={type}
          onChange={(event) => setType(event.target.value as ListingType | "")}
          options={typeOptions}
          aria-label="Filtrer par type de bien"
          className="max-w-xs"
        />
      </div>

      {error && (
        <Alert variant="error" title="Impossible de charger les annonces">
          {error instanceof ApiError ? error.message : "Réessayez dans quelques instants."}
        </Alert>
      )}

      {isPending ? (
        <div className="flex justify-center py-16">
          <Spinner size="lg" label="Chargement des annonces…" />
        </div>
      ) : data && data.data.length > 0 ? (
        <div
          className={cn(
            "grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3",
            isPlaceholderData && "opacity-60"
          )}
        >
          {data.data.map((listing) => (
            <ListingCard key={listing.id} listing={listing} />
          ))}
        </div>
      ) : data ? (
        <p className="py-16 text-center text-text-muted">
          {hasActiveFilters
            ? "Aucun résultat pour ces filtres."
            : "Aucun bien disponible pour le moment."}
        </p>
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
