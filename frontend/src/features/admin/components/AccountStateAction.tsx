import { useState } from "react";
import { useIsMutating } from "@tanstack/react-query";
import { Ban, RotateCcw } from "lucide-react";
import { Alert, Button, Modal } from "../../../shared/components";
import { adminUserMutationKey, type AdminUser } from "../api";
import { useUpdateAdminUserState } from "../hooks/useUpdateAdminUserState";

interface AccountStateActionProps {
  user: AdminUser;
}

function AccountStateAction({ user }: AccountStateActionProps) {
  const [isOpen, setIsOpen] = useState(false);
  const stateChange = useUpdateAdminUserState(user.id);
  const isRowBusy = useIsMutating({ mutationKey: adminUserMutationKey(user.id) }) > 0;

  // One button for both states, so it keeps the focus when the state flips.
  const label = user.is_active ? "Suspend" : "Reactivate";
  const Icon = user.is_active ? Ban : RotateCcw;

  function handleOpen() {
    stateChange.reset();
    setIsOpen(true);
  }

  function handleClose() {
    if (stateChange.isPending) return;
    setIsOpen(false);
  }

  return (
    <>
      <button
        type="button"
        onClick={user.is_active ? handleOpen : () => stateChange.mutate(true)}
        disabled={isRowBusy}
        aria-label={`${label} ${user.email}`}
        className="flex items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900 disabled:cursor-not-allowed disabled:opacity-60"
      >
        <Icon className="size-4" aria-hidden="true" />
        {label}
      </button>

      <Modal
        isOpen={isOpen}
        onClose={handleClose}
        title="Suspend this account?"
        footer={
          <>
            <Button variant="secondary" onClick={handleClose} disabled={stateChange.isPending}>
              Cancel
            </Button>
            <Button
              variant="danger"
              onClick={() => stateChange.mutate(false, { onSuccess: () => setIsOpen(false) })}
              isLoading={stateChange.isPending}
            >
              Suspend
            </Button>
          </>
        }
      >
        <p className="mb-2 font-bold break-words text-ink-900">{user.email}</p>
        <p className="text-sm text-text">
          This user will be signed out within seconds and won't be able to sign in. Their listings
          stay hidden until the account is reactivated.
        </p>
        {stateChange.isError && (
          <Alert variant="error" className="mt-4">
            Couldn't suspend the account. Please try again.
          </Alert>
        )}
      </Modal>
    </>
  );
}

export { AccountStateAction };
