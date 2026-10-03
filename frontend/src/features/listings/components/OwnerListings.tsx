import { useEffect } from "react";
import { Link, useNavigate, useSearchParams } from "react-router";
import { Alert, DimensionRule, EmptyState, Pagination, Skeleton } from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";
import { useOwnerListings } from "../hooks/useOwnerListings";
import { parsePageParam } from "../ownerListingsPage";
import { OwnerListingRow } from "./OwnerListingRow";

const CREATE_LISTING_PATH = "/owner/listings/new";
const SKELETON_ROW_COUNT = 4;

function OwnerListings() {
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();
  const page = parsePageParam(searchParams.get("page"));

  const { data, isPending, isPlaceholderData, error } = useOwnerListings({ page });
  const totalPages = data?.pagination.total_pages ?? 0;

  // A page past the end (e.g. after deletions) is swapped for the last one, without a history entry.
  useEffect(() => {
    if (totalPages > 0 && page > totalPages) {
      setSearchParams({ page: String(totalPages) }, { replace: true });
    }
  }, [page, totalPages, setSearchParams]);

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-6 px-4 py-8 sm:px-7">
      <div className="flex flex-wrap items-end justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-ink-900">My properties</h1>
          <DimensionRule width={120} className="mt-3.5" />
        </div>
        <Link
          to={CREATE_LISTING_PATH}
          className="rounded-sm border border-brass-600 bg-primary px-4 py-3 text-sm font-semibold text-ink-900 hover:bg-brass-600 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
        >
          + Publish a listing
        </Link>
      </div>

      {error && (
        <Alert variant="error" title="Unable to load your properties">
          Please try again in a moment.
        </Alert>
      )}

      {isPending ? (
        <div role="status">
          <span className="sr-only">Loading your properties…</span>
          <ul className="flex flex-col gap-3" aria-hidden="true">
            {Array.from({ length: SKELETON_ROW_COUNT }).map((_, index) => (
              <li
                key={index}
                className="flex items-center gap-4 border border-border bg-surface p-3"
              >
                <Skeleton variant="block" className="h-18 w-24 flex-none" />
                <div className="flex flex-1 flex-col gap-2">
                  <Skeleton variant="line" className="w-1/2" />
                  <Skeleton variant="line" className="w-1/3" />
                  <Skeleton variant="line" className="w-1/4" />
                </div>
              </li>
            ))}
          </ul>
        </div>
      ) : data && data.pagination.total === 0 ? (
        <EmptyState
          title="You haven't listed a property yet"
          description="Create a listing, add photos, then publish it so seekers can find it."
          primaryAction={{
            label: "Publish your first listing",
            onClick: () => navigate(CREATE_LISTING_PATH),
          }}
        />
      ) : data ? (
        <ul className={cn("flex flex-col gap-3", isPlaceholderData && "opacity-60")}>
          {data.data.map((listing) => (
            <OwnerListingRow key={listing.id} listing={listing} />
          ))}
        </ul>
      ) : null}

      {totalPages > 1 && (
        <Pagination
          page={page}
          totalPages={totalPages}
          onPageChange={(nextPage) => setSearchParams({ page: String(nextPage) })}
          className="self-center"
        />
      )}
    </div>
  );
}

export { OwnerListings };
