import { useState } from "react";
import { Link } from "react-router";
import {
  Alert,
  Badge,
  Card,
  EmptyState,
  Pagination,
  Select,
  Spinner,
} from "../../../shared/components";
import { useOwnerRequestQueue } from "../hooks/useOwnerRequestQueue";
import { ownerRequestStatusBadge } from "../ownerRequestStatusBadge";

const STATUS_OPTIONS = [
  { value: "", label: "All statuses" },
  { value: "pending", label: "Pending" },
  { value: "approved", label: "Approved" },
  { value: "rejected", label: "Rejected" },
];

function OwnerRequestQueueList() {
  const [status, setStatus] = useState("pending");
  const [page, setPage] = useState(1);

  const { data, isPending, error } = useOwnerRequestQueue({
    status: status || undefined,
    page,
  });

  function handleStatusChange(nextStatus: string) {
    setStatus(nextStatus);
    setPage(1);
  }

  return (
    <div className="mx-auto w-full max-w-3xl px-4 py-10">
      <div className="mb-6 flex items-center justify-between gap-4">
        <h1 className="text-2xl font-bold text-ink-900">Owner requests</h1>
        <Select
          aria-label="Filter by status"
          className="w-44"
          value={status}
          options={STATUS_OPTIONS}
          onChange={(event) => handleStatusChange(event.target.value)}
        />
      </div>

      {isPending && (
        <div className="flex justify-center py-16">
          <Spinner size="lg" label="Loading…" />
        </div>
      )}

      {error && <Alert variant="error">The queue is unavailable. Please try again.</Alert>}

      {!isPending && !error && data && data.data.length === 0 && (
        <EmptyState title="No requests" description="No owner request matches this filter." />
      )}

      {!isPending && !error && data && data.data.length > 0 && (
        <div className="flex flex-col gap-3">
          {data.data.map((request) => {
            const badge = ownerRequestStatusBadge(request.status);
            return (
              <Link key={request.id} to={`/admin/owner-requests/${request.id}`}>
                <Card
                  padding="sm"
                  className="flex items-center justify-between gap-4 transition hover:border-border-strong"
                >
                  <div className="flex flex-col gap-1">
                    <span className="font-semibold text-text">{request.full_name}</span>
                    <span className="text-sm text-text-muted">{request.email}</span>
                    <span className="text-sm text-text-muted">{request.phone}</span>
                  </div>
                  <div className="flex flex-col items-end gap-2">
                    <Badge tone={badge.tone}>{badge.label}</Badge>
                    <span className="text-xs text-text-muted">
                      {new Date(request.created_at).toLocaleDateString()}
                    </span>
                  </div>
                </Card>
              </Link>
            );
          })}

          <Pagination
            page={data.pagination.page}
            totalPages={data.pagination.total_pages}
            onPageChange={setPage}
            className="mt-2 justify-center"
          />
        </div>
      )}
    </div>
  );
}

export { OwnerRequestQueueList };
