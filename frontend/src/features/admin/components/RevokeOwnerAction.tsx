import { useState } from "react";
import { useIsMutating } from "@tanstack/react-query";
import { UserMinus } from "lucide-react";
import { Alert, Button, Modal } from "../../../shared/components";
import { adminUserMutationKey, type AdminUser } from "../api";
import { useRevokeOwnerRole } from "../hooks/useRevokeOwnerRole";

interface RevokeOwnerActionProps {
  user: AdminUser;
}

function RevokeOwnerAction({ user }: RevokeOwnerActionProps) {
  const [isOpen, setIsOpen] = useState(false);
  const revoke = useRevokeOwnerRole(user.id);
  const isRowBusy = useIsMutating({ mutationKey: adminUserMutationKey(user.id) }) > 0;

  function handleOpen() {
    revoke.reset();
    setIsOpen(true);
  }

  function handleClose() {
    if (revoke.isPending) return;
    setIsOpen(false);
  }

  return (
    <>
      <button
        type="button"
        onClick={handleOpen}
        disabled={isRowBusy}
        aria-label={`Revoke owner role of ${user.email}`}
        className="flex items-center gap-1 text-sm font-semibold whitespace-nowrap text-ink-500 hover:text-ink-900 disabled:cursor-not-allowed disabled:opacity-60"
      >
        <UserMinus className="size-4" aria-hidden="true" />
        Revoke owner role
      </button>

      <Modal
        isOpen={isOpen}
        onClose={handleClose}
        title="Revoke this owner role?"
        footer={
          <>
            <Button variant="secondary" onClick={handleClose} disabled={revoke.isPending}>
              Cancel
            </Button>
            <Button variant="danger" onClick={() => revoke.mutate()} isLoading={revoke.isPending}>
              Revoke owner role
            </Button>
          </>
        }
      >
        <p className="mb-2 font-bold break-words text-ink-900">{user.email}</p>
        <p className="text-sm text-text">
          This user becomes a seeker. Their listings are hidden but kept, and they can apply to
          become an owner again.
        </p>
        {revoke.isError && (
          <Alert variant="error" className="mt-4">
            Couldn't revoke the owner role. Please try again.
          </Alert>
        )}
      </Modal>
    </>
  );
}

export { RevokeOwnerAction };
