import { useState } from "react";
import { Alert, Button, Modal } from "../../../shared/components";
import { usePublishListing } from "../hooks/usePublishListing";
import { isPhotoRequired } from "../listingPublication";

interface PublishListingActionProps {
  listingId: string;
  hasPhoto: boolean;
  describedById: string;
}

function PublishListingAction({ listingId, hasPhoto, describedById }: PublishListingActionProps) {
  const [isOpen, setIsOpen] = useState(false);
  const publication = usePublishListing(listingId);

  function handleOpen() {
    publication.reset();
    setIsOpen(true);
  }

  function handleClose() {
    if (publication.isPending) return;
    setIsOpen(false);
  }

  function handlePublish() {
    publication.mutate(undefined, {
      onError: (error) => {
        if (isPhotoRequired(error)) setIsOpen(false);
      },
    });
  }

  return (
    <>
      <Button size="sm" onClick={handleOpen} disabled={!hasPhoto} aria-describedby={describedById}>
        Publish
      </Button>

      <Modal
        isOpen={isOpen}
        onClose={handleClose}
        title="Publish this listing?"
        footer={
          <>
            <Button variant="secondary" onClick={handleClose} disabled={publication.isPending}>
              Cancel
            </Button>
            <Button onClick={handlePublish} isLoading={publication.isPending}>
              Publish
            </Button>
          </>
        }
      >
        <p className="text-sm text-text">
          It will be visible to everyone. Publishing can't be undone: afterwards you can only mark
          it unavailable or delete it.
        </p>
        {publication.isError && !isPhotoRequired(publication.error) && (
          <Alert variant="error" className="mt-4">
            Couldn't publish the listing. Please try again.
          </Alert>
        )}
      </Modal>
    </>
  );
}

export { PublishListingAction };
