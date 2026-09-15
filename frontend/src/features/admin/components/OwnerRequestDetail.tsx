import { useState } from "react";
import { useParams } from "react-router";
import {
  Alert,
  Badge,
  Button,
  Card,
  Modal,
  Spinner,
  TextArea,
  useToast,
} from "../../../shared/components";
import { useOwnerRequestDetail } from "../hooks/useOwnerRequestDetail";
import { useReviewOwnerRequest } from "../hooks/useReviewOwnerRequest";
import { ownerRequestStatusBadge } from "../ownerRequestStatusBadge";
import { reviewErrorMessage } from "../reviewErrorMessage";
import { DocumentViewer } from "./DocumentViewer";

function OwnerRequestDetail() {
  const { id } = useParams<{ id: string }>();
  const { data, isPending, error } = useOwnerRequestDetail(id);
  const reviewMutation = useReviewOwnerRequest();
  const { showToast } = useToast();
  const [adminNote, setAdminNote] = useState("");
  const [isRejectConfirmOpen, setIsRejectConfirmOpen] = useState(false);

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
  const isPendingReview = data.status === "pending";
  const isReviewing = reviewMutation.isPending;
  const requestId = data.id;

  function handleApprove() {
    reviewMutation.mutate(
      { id: requestId, status: "approved", adminNote: adminNote || undefined },
      {
        onSuccess: () => showToast("Request approved.", { variant: "success" }),
        onError: (err) => showToast(reviewErrorMessage(err), { variant: "error" }),
      }
    );
  }

  function handleRejectConfirm() {
    reviewMutation.mutate(
      { id: requestId, status: "rejected", adminNote: adminNote || undefined },
      {
        onSuccess: () => showToast("Request rejected.", { variant: "success" }),
        onError: (err) => showToast(reviewErrorMessage(err), { variant: "error" }),
        onSettled: () => setIsRejectConfirmOpen(false),
      }
    );
  }

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

      <Card className="mb-4 flex flex-col gap-4">
        <h2 className="text-sm font-semibold text-text-muted">Documents</h2>
        <div className="flex flex-wrap gap-6">
          {data.documents.map((doc) => (
            <DocumentViewer key={doc.doc_id} requestId={data.id} doc={doc} />
          ))}
        </div>
      </Card>

      {isPendingReview && (
        <Card className="flex flex-col gap-4">
          <h2 className="text-sm font-semibold text-text-muted">Decision</h2>
          <TextArea
            aria-label="Admin note"
            placeholder="Optional note — shared with the applicant."
            value={adminNote}
            onChange={(event) => setAdminNote(event.target.value)}
            rows={3}
            disabled={isReviewing}
          />
          <div className="flex justify-end gap-3">
            <Button
              variant="danger-outline"
              onClick={() => setIsRejectConfirmOpen(true)}
              disabled={isReviewing}
            >
              Reject
            </Button>
            <Button
              onClick={handleApprove}
              isLoading={isReviewing && reviewMutation.variables?.status === "approved"}
              disabled={isReviewing}
            >
              Approve
            </Button>
          </div>
        </Card>
      )}

      <Modal
        isOpen={isRejectConfirmOpen}
        onClose={() => !isReviewing && setIsRejectConfirmOpen(false)}
        title="Reject this request?"
        footer={
          <>
            <Button
              variant="secondary"
              onClick={() => setIsRejectConfirmOpen(false)}
              disabled={isReviewing}
            >
              Cancel
            </Button>
            <Button
              variant="danger"
              onClick={handleRejectConfirm}
              isLoading={isReviewing && reviewMutation.variables?.status === "rejected"}
              disabled={isReviewing}
            >
              Reject request
            </Button>
          </>
        }
      >
        <p className="text-sm text-text">
          The applicant will be notified by email. This cannot be undone.
        </p>
      </Modal>
    </div>
  );
}

export { OwnerRequestDetail };
