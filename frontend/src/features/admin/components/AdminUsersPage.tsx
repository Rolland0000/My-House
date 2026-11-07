import { useMemo } from "react";
import { useSearchParams } from "react-router";
import { Alert, Button, EmptyState, Pagination, Select, Spinner } from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";
import { ROLE_BADGE, STATE_BADGE } from "../adminUserBadges";
import {
  parseAdminUsersCriteria,
  ROLES,
  STATES,
  toAdminUsersRequest,
  toAdminUsersUrlParams,
  type AdminUsersCriteria,
} from "../adminUsersCriteria";
import { useAdminUsers } from "../hooks/useAdminUsers";
import { AdminUserRow } from "./AdminUserRow";
import { AdminUsersSearch } from "./AdminUsersSearch";

const ROLE_OPTIONS = [
  { value: "", label: "All roles" },
  ...ROLES.map((role) => ({ value: role, label: ROLE_BADGE[role].label })),
];

const STATE_OPTIONS = [
  { value: "", label: "All states" },
  ...STATES.map((state) => ({ value: state, label: STATE_BADGE[state].label })),
];

const COLUMNS = ["Name", "Email", "Phone", "Role", "State", "Listings", "Joined"];

function AdminUsersPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const criteria = useMemo(() => parseAdminUsersCriteria(searchParams), [searchParams]);
  const request = useMemo(() => toAdminUsersRequest(criteria), [criteria]);
  const { data, isPending, isFetching, isPlaceholderData, error, refetch } = useAdminUsers(request);

  function navigate(next: AdminUsersCriteria, replace = false) {
    const nextParams = toAdminUsersUrlParams(next);
    if (nextParams.toString() === searchParams.toString()) return;
    // flushSync renders the new URL at once, so a field resynced from it can't drop typed text.
    setSearchParams(nextParams, { replace, flushSync: true });
  }

  function changeFilter(patch: Partial<AdminUsersCriteria>, replace = true) {
    navigate({ ...criteria, ...patch, page: 1 }, replace);
  }

  return (
    <div className="mx-auto flex w-full max-w-6xl flex-col gap-6 px-4 py-10">
      <h1 className="text-2xl font-bold text-ink-900">Users</h1>

      <div className="flex flex-wrap items-center gap-3">
        <AdminUsersSearch
          value={criteria.q}
          onSearch={(q, settled) => changeFilter({ q }, settled)}
        />
        <Select
          aria-label="Filter by role"
          className="w-40"
          value={criteria.role}
          options={ROLE_OPTIONS}
          onChange={(event) =>
            changeFilter({ role: event.target.value as AdminUsersCriteria["role"] })
          }
        />
        <Select
          aria-label="Filter by state"
          className="w-40"
          value={criteria.state}
          options={STATE_OPTIONS}
          onChange={(event) =>
            changeFilter({ state: event.target.value as AdminUsersCriteria["state"] })
          }
        />
      </div>

      <div className="flex min-h-5 items-center gap-2 text-sm font-medium text-ink-500">
        <p aria-live="polite">
          {data && `${data.pagination.total} account${data.pagination.total === 1 ? "" : "s"}`}
        </p>
        {isPlaceholderData && <Spinner size="sm" label="Updating results…" />}
      </div>

      {error && (
        <Alert variant="error" title="Unable to load accounts">
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
      ) : data && data.data.length > 0 ? (
        <>
          {/* Focusable so keyboard users can scroll the table sideways at tablet width.
              `relative` makes it clip the sr-only elements, which would otherwise widen the page. */}
          <div
            role="region"
            aria-label="Users table"
            aria-busy={isPlaceholderData}
            tabIndex={0}
            className={cn(
              "relative overflow-x-auto border border-border bg-surface focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus",
              isPlaceholderData && "opacity-60"
            )}
          >
            <table className="w-full min-w-4xl text-left text-sm">
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
                {data.data.map((user) => (
                  <AdminUserRow key={user.id} user={user} />
                ))}
              </tbody>
            </table>
          </div>

          <Pagination
            page={data.pagination.page}
            totalPages={data.pagination.total_pages}
            onPageChange={(page) => navigate({ ...criteria, page })}
            className="justify-center"
          />
        </>
      ) : data && data.pagination.total > 0 ? (
        // A page past the last one, e.g. after deleting its only row.
        <EmptyState
          title="No accounts on this page"
          secondaryAction={{
            label: "Go to page 1",
            onClick: () => navigate({ ...criteria, page: 1 }),
          }}
        />
      ) : data ? (
        <EmptyState title="No accounts" description="No account matches these filters." />
      ) : null}
    </div>
  );
}

export { AdminUsersPage };
