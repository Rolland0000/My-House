import { Button, Modal } from "../../../shared/components";

interface OwnerActivationModalProps {
  isOpen: boolean;
  onSignInAgain: () => void;
  onClose: () => void;
}

function OwnerActivationModal({ isOpen, onSignInAgain, onClose }: OwnerActivationModalProps) {
  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Your owner account is ready"
      size="sm"
      footer={<Button onClick={onSignInAgain}>Sign in again</Button>}
    >
      <p className="text-sm text-text-muted">
        Your request was approved. Sign in again to start managing your properties.
      </p>
    </Modal>
  );
}

export { OwnerActivationModal };
export type { OwnerActivationModalProps };
