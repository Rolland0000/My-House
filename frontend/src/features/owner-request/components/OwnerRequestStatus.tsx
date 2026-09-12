import { Link, useNavigate } from "react-router";
import { Alert, Card, EmptyState, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useOwnerRequestStatus } from "../hooks/useOwnerRequestStatus";

/**
 * Minimal placeholder: only renders the "pending" state. Approved/rejected
 * views belong to a later ticket that extends this page.
 */
function OwnerRequestStatus() {
  const navigate = useNavigate();
  const { data, isPending, error } = useOwnerRequestStatus();

  if (isPending) {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" label="Loading…" />
      </div>
    );
  }

  const notFound = error instanceof ApiError && error.status === 404;

  return (
    <div className="mx-auto w-full max-w-md px-4 py-12">
      <Card>
        {notFound && (
          <EmptyState
            title="No request on file"
            description="Submit an owner request from your profile to see its status here."
            primaryAction={{ label: "Go to profile", onClick: () => navigate("/profile") }}
          />
        )}
        {error && !notFound && <Alert variant="error">Status unavailable. Please try again.</Alert>}
        {data && (
          <div className="flex flex-col gap-3">
            <h1 className="text-2xl font-bold text-ink-900">Your request is pending</h1>
            <p className="text-sm text-text-muted">
              Our team is reviewing your information. You'll be notified by email once a decision is
              made.
            </p>
            <p className="text-sm text-text-muted">
              Submitted on {new Date(data.created_at).toLocaleDateString()}.
            </p>
            <Link to="/profile" className="text-sm font-semibold text-primary underline">
              Back to profile
            </Link>
          </div>
        )}
      </Card>
    </div>
  );
}

export { OwnerRequestStatus };
