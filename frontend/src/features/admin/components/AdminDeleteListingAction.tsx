import { useState } from "react";
import { Trash2 } from "lucide-react";
import { Alert, Button, Modal } from "../../../shared/components";
import { useDeleteAdminListing } from "../hooks/useDeleteAdminListing";

interface AdminDeleteListingActionProps {
  listingId: string;
  listingTitle: string;
  disabled: boolean;
}

function AdminDeleteListingAction({
  listingId,
  listingTitle,
  disabled,
}: AdminDeleteListingActionProps) {
  const [isOpen, setIsOpen] = useState(false);
  const deletion = useDeleteAdminListing(listingId);

  function handleOpen() {
    deletion.reset();
    setIsOpen(true);
  }

  function handleClose() {
    if (deletion.isPending) return;
    setIsOpen(false);
  }

  function handleConfirm() {
    // The row normally unmounts first; this covers a failed refetch that leaves it on screen.
    deletion.mutate(undefined, { onSuccess: () => setIsOpen(false) });
  }

  return (
    <>
      <button
        type="button"
        onClick={handleOpen}
        disabled={disabled}
        aria-label={`Delete ${listingTitle}`}
        className="flex items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900 disabled:cursor-not-allowed disabled:opacity-60"
      >
        <Trash2 className="size-4" aria-hidden="true" />
        Delete
      </button>

      <Modal
        isOpen={isOpen}
        onClose={handleClose}
        title="Delete this listing?"
        footer={
          <>
            <Button variant="secondary" onClick={handleClose} disabled={deletion.isPending}>
              Cancel
            </Button>
            <Button variant="danger" onClick={handleConfirm} isLoading={deletion.isPending}>
              Delete permanently
            </Button>
          </>
        }
      >
        <p className="mb-2 font-bold break-words text-ink-900">{listingTitle}</p>
        <p className="text-sm text-text">
          This listing and all its photos will be permanently deleted. This can't be undone.
        </p>
        {deletion.isError && (
          <Alert variant="error" className="mt-4">
            Couldn't delete the listing. Please try again.
          </Alert>
        )}
      </Modal>
    </>
  );
}

export { AdminDeleteListingAction };
export type { AdminDeleteListingActionProps };
