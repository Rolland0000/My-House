import { useState } from "react";
import { useNavigate } from "react-router";
import { Alert, Button, Modal, useToast } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useAuth } from "../../auth";
import { useDeleteAccount } from "../hooks/useDeleteAccount";

const GENERIC_ERROR_MESSAGE = "Something went wrong. Please try again.";

function DeleteAccountSection() {
  const [isOpen, setIsOpen] = useState(false);
  const { clearSession } = useAuth();
  const { showToast } = useToast();
  const navigate = useNavigate();
  const deleteAccount = useDeleteAccount();

  function handleClose() {
    if (deleteAccount.isPending) return;
    setIsOpen(false);
  }

  function handleConfirm() {
    deleteAccount.mutate(undefined, {
      onSuccess: () => {
        clearSession();
        navigate("/", { replace: true });
        showToast("Your account has been deleted.", { variant: "success" });
      },
    });
  }

  const requestError = deleteAccount.error instanceof ApiError ? deleteAccount.error : null;

  return (
    <section className="flex flex-col gap-3 border-t border-border pt-6">
      <div>
        <h2 className="text-base font-semibold text-text">Danger zone</h2>
        <p className="text-sm text-text-muted">
          Deleting your account is permanent and cannot be undone.
        </p>
      </div>

      <Button type="button" variant="danger" className="self-start" onClick={() => setIsOpen(true)}>
        Delete account
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
            <Button variant="danger" onClick={handleConfirm} isLoading={deleteAccount.isPending}>
              Yes, delete my account
            </Button>
          </>
        }
      >
        <p className="text-sm text-text">
          This action is permanent. Your listings, photos, and profile information will be removed
          along with your account.
        </p>
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
