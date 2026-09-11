import { useEffect, useState } from "react";
import { Check } from "lucide-react";
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
  const [justVerified, setJustVerified] = useState(false);
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
        onSuccess: ({ data }) => {
          // The one orchestrated animation in the product: a brief brass-stamp
          // confirmation before moving on. Collapses to ~0ms under
          // prefers-reduced-motion (see index.css).
          setJustVerified(true);
          const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
          window.setTimeout(
            () => onVerified(data.registration_ticket ?? null),
            reduceMotion ? 0 : 450
          );
        },
        onError: () => setCode(emptyCode()),
      }
    );
  }

  if (justVerified) {
    return (
      <div className="flex items-center gap-3.5 py-2">
        <span className="flex size-13 flex-none items-center justify-center border-2 border-primary animate-mh-stamp">
          <Check className="size-4 text-ink-900" strokeWidth={3} aria-hidden="true" />
        </span>
        <div>
          <p className="font-semibold text-ink-900">Email confirmed</p>
          <p className="text-sm text-text-muted">Taking you to the next step…</p>
        </div>
      </div>
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
        ← Edit email
      </button>

      <div>
        <h1 className="text-[22px] font-bold text-ink-900">Check your inbox</h1>
        <p className="text-sm text-text-muted">
          Code sent to <span className="font-semibold text-text">{email}</span>
        </p>
      </div>

      {isInvalid && (
        <Alert variant="error">
          Invalid or expired code. Check the code you received by email, or request a new one.
        </Alert>
      )}

      {isResendRateLimited && (
        <Alert variant="warning">
          Too many resend requests. Try again in {formatCountdown(resendCooldown)}.
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
        Verify
      </Button>

      <p className="text-sm text-text-muted">
        Didn't receive anything?{" "}
        <button
          type="button"
          onClick={handleResend}
          disabled={resendDisabled}
          className="font-semibold text-primary disabled:cursor-not-allowed disabled:opacity-50"
        >
          {resendCooldown > 0 ? `Resend code (${formatCountdown(resendCooldown)})` : "Resend code"}
        </button>
      </p>
    </div>
  );
}

export { OtpVerifyForm };
export type { OtpVerifyFormProps };
