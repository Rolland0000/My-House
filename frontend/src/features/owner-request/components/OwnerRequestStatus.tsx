import { Link, useNavigate } from "react-router";
import { Alert, Badge, Card, EmptyState, Spinner } from "../../../shared/components";
import { useOwnerRequestStatus } from "../hooks/useOwnerRequestStatus";
import { ownerRequestStatusView } from "../ownerRequestStatusView";

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

  return (
    <div className="mx-auto w-full max-w-md px-4 py-12">
      <Card>
        {error && <Alert variant="error">Status unavailable. Please try again.</Alert>}
        {!error && data === null && (
          <EmptyState
            title="No request on file"
            description="Submit an owner request from your profile to see its status here."
            primaryAction={{ label: "Go to profile", onClick: () => navigate("/profile") }}
          />
        )}
        {!error &&
          data &&
          (() => {
            const view = ownerRequestStatusView(data);
            return (
              <div className="flex flex-col gap-3">
                <Badge tone={view.badgeTone}>{view.badgeLabel}</Badge>
                <h1 className="text-2xl font-bold text-ink-900">{view.title}</h1>
                <p className="text-sm text-text-muted">{view.description}</p>
                {view.adminNote && (
                  <p className="text-sm text-text-muted">
                    <span className="font-semibold">Admin note:</span> {view.adminNote}
                  </p>
                )}
                <p className="text-sm text-text-muted">
                  Submitted on {new Date(data.created_at).toLocaleDateString()}.
                </p>
                <Link
                  to={view.showCta ? "/owner-request" : "/profile"}
                  className="text-sm font-semibold text-primary underline"
                >
                  {view.showCta ? "Submit a new request" : "Back to profile"}
                </Link>
              </div>
            );
          })()}
      </Card>
    </div>
  );
}

export { OwnerRequestStatus };
