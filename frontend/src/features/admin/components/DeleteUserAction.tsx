import { useState } from "react";
import { useIsMutating } from "@tanstack/react-query";
import { Trash2 } from "lucide-react";
import { Alert, Button, Modal } from "../../../shared/components";
import { adminUserMutationKey, type AdminUser } from "../api";
import { useDeleteAdminUser } from "../hooks/useDeleteAdminUser";

interface DeleteUserActionProps {
  user: AdminUser;
}

function DeleteUserAction({ user }: DeleteUserActionProps) {
  const [isOpen, setIsOpen] = useState(false);
  const deletion = useDeleteAdminUser(user.id);
  const isRowBusy = useIsMutating({ mutationKey: adminUserMutationKey(user.id) }) > 0;

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
        disabled={isRowBusy}
        aria-label={`Delete ${user.email}`}
        className="flex items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900 disabled:cursor-not-allowed disabled:opacity-60"
      >
        <Trash2 className="size-4" aria-hidden="true" />
        Delete
      </button>

      <Modal
        isOpen={isOpen}
        onClose={handleClose}
        title="Delete this account?"
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
        <p className="mb-2 font-bold break-words text-ink-900">{user.email}</p>
        <p className="text-sm text-text">
          This account, its listings, photos and documents will be permanently deleted. This can't
          be undone.
        </p>
        {deletion.isError && (
          <Alert variant="error" className="mt-4">
            Couldn't delete the account. Please try again.
          </Alert>
        )}
      </Modal>
    </>
  );
}

export { DeleteUserAction };
