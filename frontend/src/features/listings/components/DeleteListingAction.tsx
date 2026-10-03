import { useState } from "react";
import { useIsMutating } from "@tanstack/react-query";
import { Trash2 } from "lucide-react";
import { Alert, Button, Modal } from "../../../shared/components";
import { useDeleteListing } from "../hooks/useDeleteListing";
import { LISTING_STATUS_MUTATION_KEY } from "../hooks/useUpdateListingStatus";

interface DeleteListingActionProps {
  listingId: string;
  listingTitle: string;
}

function DeleteListingAction({ listingId, listingTitle }: DeleteListingActionProps) {
  const [isOpen, setIsOpen] = useState(false);
  const deletion = useDeleteListing(listingId);
  // A delete landing first would make that status change end in a 404 and a second toast.
  const isStatusPending =
    useIsMutating({ mutationKey: [...LISTING_STATUS_MUTATION_KEY, listingId] }) > 0;

  function handleOpen() {
    deletion.reset();
    setIsOpen(true);
  }

  function handleClose() {
    if (deletion.isPending) return;
    setIsOpen(false);
  }

  return (
    <>
      <button
        type="button"
        onClick={handleOpen}
        disabled={isStatusPending}
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
            <Button
              variant="danger"
              onClick={() => deletion.mutate()}
              isLoading={deletion.isPending}
            >
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

export { DeleteListingAction };
export type { DeleteListingActionProps };
