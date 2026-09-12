import { Link } from "react-router";
import { Alert, Badge, Spinner } from "../../../shared/components";
import { ownerRequestStatusView, useOwnerRequestStatus } from "../../owner-request";
import type { Profile } from "../api";

interface OwnerRequestStatusBlockProps {
  role: Profile["role"];
}

/**
 * Hidden for owner or admin with no request on record: neither can apply
 * (admin is blocked server-side), so there's nothing to show or act on.
 */
function OwnerRequestStatusBlock({ role }: OwnerRequestStatusBlockProps) {
  const { data, isPending, error } = useOwnerRequestStatus();

  if (isPending) {
    return (
      <div className="flex justify-center border-b border-border pb-6">
        <Spinner size="sm" label="Loading request status…" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="border-b border-border pb-6">
        <Alert variant="error">Owner request status unavailable. Please try again.</Alert>
      </div>
    );
  }

  if (data === undefined) return null;
  if ((role === "owner" || role === "admin") && data === null) return null;

  const view = ownerRequestStatusView(data);

  return (
    <div className="flex flex-col gap-2 border-b border-border pb-6">
      <Badge tone={view.badgeTone}>{view.badgeLabel}</Badge>
      <h2 className="text-base font-semibold text-ink-900">{view.title}</h2>
      <p className="text-sm text-text-muted">{view.description}</p>
      {view.adminNote && (
        <p className="text-sm text-text-muted">
          <span className="font-semibold">Admin note:</span> {view.adminNote}
        </p>
      )}
      {view.showCta && (
        <Link to="/owner-request" className="text-sm font-semibold text-primary underline">
          {view.state === "rejected" ? "Submit a new request" : "Become an owner"}
        </Link>
      )}
    </div>
  );
}

export { OwnerRequestStatusBlock };
