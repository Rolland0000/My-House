import { useState } from "react";
import { Button, Modal, TextArea, useToast } from "../../../shared/components";

const REASONS = [
  "Price doesn't match the photos",
  "Property no longer available",
  "Suspected scam",
  "Other",
] as const;

interface ReportListingModalProps {
  isOpen: boolean;
  onClose: () => void;
  listingTitle: string;
}

/**
 * UI-only for now: there is no report endpoint yet (`features/contact/api.ts` is
 * empty). Submitting shows a local confirmation toast and closes — wiring this to
 * a real `POST /listings/:id/report` is a follow-up backend ticket.
 */
function ReportListingModal({ isOpen, onClose, listingTitle }: ReportListingModalProps) {
  const [reason, setReason] = useState<(typeof REASONS)[number]>(REASONS[0]);
  const [detail, setDetail] = useState("");
  const { showToast } = useToast();

  function handleClose() {
    setReason(REASONS[0]);
    setDetail("");
    onClose();
  }

  function handleSubmit() {
    showToast("Thanks — a moderator will review this listing.", { variant: "success" });
    handleClose();
  }

  return (
    <Modal
      isOpen={isOpen}
      onClose={handleClose}
      title="Report this listing"
      footer={
        <>
          <Button variant="secondary" onClick={handleClose}>
            Cancel
          </Button>
          <Button onClick={handleSubmit}>Submit report</Button>
        </>
      }
    >
      <p className="mb-4 text-sm text-text-muted">
        Tell us what's wrong with "{listingTitle}". A moderator reviews every report.
      </p>
      <div className="mb-4 flex flex-col gap-2">
        {REASONS.map((option) => (
          <label
            key={option}
            className={
              "flex items-center gap-2.5 rounded-sm border px-3 py-2.5 text-sm font-medium text-text " +
              (reason === option ? "border-ink-600 bg-primary-soft" : "border-border")
            }
          >
            <input
              type="radio"
              name="report-reason"
              checked={reason === option}
              onChange={() => setReason(option)}
              className="accent-ink-600"
            />
            {option}
          </label>
        ))}
      </div>
      <TextArea
        placeholder="Optional — add detail that would help a reviewer."
        value={detail}
        onChange={(event) => setDetail(event.target.value)}
        rows={3}
      />
    </Modal>
  );
}

export { ReportListingModal };
export type { ReportListingModalProps };
