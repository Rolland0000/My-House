import { useEffect, useState } from "react";
import { Alert, Button } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useCountdown } from "../../../shared/hooks/useCountdown";
import { formatCountdown } from "../../../shared/utils/format";
import { useOtpRequest } from "../hooks/useOtpRequest";
import { useOtpVerify } from "../hooks/useOtpVerify";
import {
  OTP_CODE_LENGTH,
  OTP_RATE_LIMIT_COOLDOWN_SECONDS,
  OTP_RESEND_INITIAL_COOLDOWN_SECONDS,
} from "../constants";
import { OtpCodeInput } from "./OtpCodeInput";

interface OtpVerifyFormProps {
  email: string;
  onBack: () => void;
  /** Registration ticket for a new user, `null` when the session is already open. */
  onVerified: (registrationTicket: string | null) => void;
}

function emptyCode(): string[] {
  return Array(OTP_CODE_LENGTH).fill("");
}

function OtpVerifyForm({ email, onBack, onVerified }: OtpVerifyFormProps) {
  const [code, setCode] = useState<string[]>(emptyCode);
  const verifyOtp = useOtpVerify();
  const resendOtp = useOtpRequest();
  const { secondsLeft: resendCooldown, start: startResendCooldown } = useCountdown();

  useEffect(() => {
    startResendCooldown(OTP_RESEND_INITIAL_COOLDOWN_SECONDS);
  }, [startResendCooldown]);

  const isInvalid = verifyOtp.error instanceof ApiError && verifyOtp.error.code === "OTP_INVALID";
  const isResendRateLimited =
    resendOtp.error instanceof ApiError && resendOtp.error.code === "OTP_RATE_LIMITED";
  const resendDisabled = resendCooldown > 0 || resendOtp.isPending;

  function handleComplete(fullCode: string) {
    verifyOtp.mutate(
      { email, code: fullCode },
      {
        onSuccess: ({ data }) => onVerified(data.registration_ticket ?? null),
        onError: () => setCode(emptyCode()),
      }
    );
  }

  function handleResend() {
    resendOtp.mutate(email, {
      onSuccess: () => startResendCooldown(OTP_RESEND_INITIAL_COOLDOWN_SECONDS),
      onError: (error) => {
        if (error instanceof ApiError && error.code === "OTP_RATE_LIMITED") {
          startResendCooldown(error.retryAfterSeconds ?? OTP_RATE_LIMIT_COOLDOWN_SECONDS);
        }
      },
    });
  }

  return (
    <div className="flex flex-col gap-5">
      <button
        type="button"
        onClick={onBack}
        className="self-start text-sm text-text-muted hover:text-text"
      >
        ← Modifier l'email
      </button>

      <div>
        <h2 className="text-lg font-bold text-text">Vérifiez votre boîte mail</h2>
        <p className="text-sm text-text-muted">
          Code envoyé à <span className="font-semibold text-text">{email}</span>
        </p>
      </div>

      {isInvalid && (
        <Alert variant="error">
          Code invalide ou expiré. Vérifiez le code reçu par email ou demandez-en un nouveau.
        </Alert>
      )}

      {isResendRateLimited && (
        <Alert variant="warning">
          Trop de demandes de renvoi. Réessayez dans {formatCountdown(resendCooldown)}.
        </Alert>
      )}

      <OtpCodeInput
        length={OTP_CODE_LENGTH}
        value={code}
        onChange={setCode}
        onComplete={handleComplete}
        disabled={verifyOtp.isPending}
      />

      <Button
        type="button"
        onClick={() => handleComplete(code.join(""))}
        isLoading={verifyOtp.isPending}
        disabled={code.some((digit) => !digit)}
      >
        Vérifier
      </Button>

      <p className="text-sm text-text-muted">
        Rien reçu ?{" "}
        <button
          type="button"
          onClick={handleResend}
          disabled={resendDisabled}
          className="font-semibold text-primary disabled:cursor-not-allowed disabled:opacity-50"
        >
          {resendCooldown > 0
            ? `Renvoyer le code (${formatCountdown(resendCooldown)})`
            : "Renvoyer le code"}
        </button>
      </p>
    </div>
  );
}

export { OtpVerifyForm };
export type { OtpVerifyFormProps };
