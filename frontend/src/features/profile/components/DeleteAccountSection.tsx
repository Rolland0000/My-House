import { useState } from "react";
import { useNavigate } from "react-router";
import { Alert, Button, Input, Modal, useToast } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useAuth } from "../../auth";
import { useDeleteAccount } from "../hooks/useDeleteAccount";

const GENERIC_ERROR_MESSAGE = "Something went wrong. Please try again.";
const CONFIRM_WORD = "DELETE";

interface DeleteAccountSectionProps {
  isOwner: boolean;
}

function DeleteAccountSection({ isOwner }: DeleteAccountSectionProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [confirmText, setConfirmText] = useState("");
  const { clearSession } = useAuth();
  const { showToast } = useToast();
  const navigate = useNavigate();
  const deleteAccount = useDeleteAccount();

  function handleClose() {
    if (deleteAccount.isPending) return;
    setIsOpen(false);
    setConfirmText("");
  }

  function handleConfirm() {
    if (confirmText !== CONFIRM_WORD) return;
    deleteAccount.mutate(undefined, {
      onSuccess: () => {
        clearSession();
        navigate("/", { replace: true });
        showToast("Your account has been deleted.", { variant: "success" });
      },
    });
  }

  const requestError = deleteAccount.error instanceof ApiError ? deleteAccount.error : null;
  const canConfirm = confirmText === CONFIRM_WORD;

  return (
    <section className="flex flex-col gap-3 border border-error/40 p-5">
      <div>
        <p className="font-bold text-ink-900">Delete my account</p>
        <p className="max-w-[56ch] text-sm text-text">
          Removes your profile permanently.{" "}
          {isOwner
            ? "Your listings and photos are removed with it — this cannot be undone."
            : "This cannot be undone."}
        </p>
      </div>

      <Button
        type="button"
        variant="danger-outline"
        className="self-start"
        onClick={() => setIsOpen(true)}
      >
        Delete my account
      </Button>

      <Modal
        isOpen={isOpen}
        onClose={handleClose}
        title="Delete your account?"
        footer={
          <>
            <Button variant="secondary" onClick={handleClose} disabled={deleteAccount.isPending}>
              Cancel
            </Button>
            <Button
              variant="danger"
              onClick={handleConfirm}
              isLoading={deleteAccount.isPending}
              disabled={!canConfirm}
            >
              Delete permanently
            </Button>
          </>
        }
      >
        <p className="mb-4 text-sm text-text">
          Your profile is removed permanently.{" "}
          {isOwner
            ? "You are a verified owner — your listings and their photos are removed with it. "
            : ""}
          This cannot be undone.
        </p>
        <label className="mb-1.5 block text-sm font-semibold text-text" htmlFor="delete-confirm">
          Type {CONFIRM_WORD} to confirm
        </label>
        <Input
          id="delete-confirm"
          value={confirmText}
          onChange={(event) => setConfirmText(event.target.value)}
          autoComplete="off"
        />
        {requestError && (
          <Alert variant="error" className="mt-4">
            {GENERIC_ERROR_MESSAGE}
          </Alert>
        )}
      </Modal>
    </section>
  );
}

export { DeleteAccountSection };
export type { DeleteAccountSectionProps };
