import { Alert, Button, Modal } from "../../../shared/components";
import { COVER_REQUIRED_MESSAGE, isCoverRequiredError } from "../listingPhotos";

interface DeletePhotoModalProps {
  isOpen: boolean;
  isPending: boolean;
  error: unknown;
  onConfirm: () => void;
  onClose: () => void;
}

function DeletePhotoModal({ isOpen, isPending, error, onConfirm, onClose }: DeletePhotoModalProps) {
  function handleClose() {
    if (isPending) return;
    onClose();
  }

  return (
    <Modal
      isOpen={isOpen}
      onClose={handleClose}
      title="Delete this photo?"
      size="sm"
      footer={
        <>
          <Button variant="secondary" onClick={handleClose} disabled={isPending}>
            Cancel
          </Button>
          <Button variant="danger" onClick={onConfirm} isLoading={isPending}>
            Delete photo
          </Button>
        </>
      }
    >
      <p className="text-sm text-text">This photo will be permanently deleted.</p>
      {Boolean(error) && (
        <Alert variant="error" className="mt-4">
          {isCoverRequiredError(error)
            ? COVER_REQUIRED_MESSAGE
            : "Couldn't delete the photo. Please try again."}
        </Alert>
      )}
    </Modal>
  );
}

export { DeletePhotoModal };
export type { DeletePhotoModalProps };
