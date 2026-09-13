import { useParams } from "react-router";
import { Alert, Badge, Card, Spinner } from "../../../shared/components";
import { useOwnerRequestDetail } from "../hooks/useOwnerRequestDetail";
import { ownerRequestStatusBadge } from "../ownerRequestStatusBadge";
import { DocumentViewer } from "./DocumentViewer";

function OwnerRequestDetail() {
  const { id } = useParams<{ id: string }>();
  const { data, isPending, error } = useOwnerRequestDetail(id);

  if (!id) {
    return (
      <div className="mx-auto w-full max-w-2xl px-4 py-10">
        <Alert variant="error">This request could not be found.</Alert>
      </div>
    );
  }

  if (isPending) {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" label="Loading…" />
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="mx-auto w-full max-w-2xl px-4 py-10">
        <Alert variant="error">This request could not be found.</Alert>
      </div>
    );
  }

  const badge = ownerRequestStatusBadge(data.status);

  return (
    <div className="mx-auto w-full max-w-2xl px-4 py-10">
      <div className="mb-6 flex items-center gap-3">
        <h1 className="text-2xl font-bold text-ink-900">{data.full_name}</h1>
        <Badge tone={badge.tone}>{badge.label}</Badge>
      </div>

      <Card className="mb-4 flex flex-col gap-2">
        <h2 className="text-sm font-semibold text-text-muted">Applicant</h2>
        <p className="text-sm text-text">{data.email}</p>
        <p className="text-sm text-text">{data.phone}</p>
        {data.secondary_phone && <p className="text-sm text-text">{data.secondary_phone}</p>}
        <p className="text-xs text-text-muted">
          Submitted on {new Date(data.created_at).toLocaleDateString()}
        </p>
        {data.reviewed_at && (
          <p className="text-xs text-text-muted">
            Reviewed on {new Date(data.reviewed_at).toLocaleDateString()}
          </p>
        )}
      </Card>

      <Card className="mb-4 flex flex-col gap-2">
        <h2 className="text-sm font-semibold text-text-muted">Identity</h2>
        <p className="text-sm text-text">{data.identity_data.full_name}</p>
        <p className="text-sm text-text">
          {data.identity_data.id_type} — {data.identity_data.id_number}
        </p>
      </Card>

      {data.admin_note && (
        <Card className="mb-4 flex flex-col gap-2">
          <h2 className="text-sm font-semibold text-text-muted">Admin note</h2>
          <p className="text-sm text-text">{data.admin_note}</p>
        </Card>
      )}

      <Card className="flex flex-col gap-4">
        <h2 className="text-sm font-semibold text-text-muted">Documents</h2>
        <div className="flex flex-wrap gap-6">
          {data.documents.map((doc) => (
            <DocumentViewer key={doc.doc_id} requestId={data.id} doc={doc} />
          ))}
        </div>
      </Card>
    </div>
  );
}

export { OwnerRequestDetail };
