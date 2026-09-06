import { useState } from "react";
import type { FormEvent } from "react";
import { Alert, Button, FormField, Input } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useCountdown } from "../../../shared/hooks/useCountdown";
import { formatCountdown } from "../../../shared/utils/format";
import { useOtpRequest } from "../hooks/useOtpRequest";
import { OTP_RATE_LIMIT_COOLDOWN_SECONDS } from "../constants";

const EMAIL_PATTERN = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

interface OtpRequestFormProps {
  initialEmail?: string;
  onRequested: (email: string) => void;
}

function OtpRequestForm({ initialEmail = "", onRequested }: OtpRequestFormProps) {
  const [email, setEmail] = useState(initialEmail);
  const [formError, setFormError] = useState<string | null>(null);
  const { secondsLeft, start } = useCountdown();
  const otpRequest = useOtpRequest();

  const isCoolingDown = secondsLeft > 0;
  const genericError =
    otpRequest.error instanceof ApiError && otpRequest.error.code !== "OTP_RATE_LIMITED"
      ? otpRequest.error.message
      : null;

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const trimmedEmail = email.trim();
    if (!EMAIL_PATTERN.test(trimmedEmail)) {
      setFormError("Invalid email address.");
      return;
    }
    setFormError(null);

    otpRequest.mutate(trimmedEmail, {
      onSuccess: () => onRequested(trimmedEmail),
      onError: (error) => {
        if (error instanceof ApiError && error.code === "OTP_RATE_LIMITED") {
          start(error.retryAfterSeconds ?? OTP_RATE_LIMIT_COOLDOWN_SECONDS);
        }
      },
    });
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-5">
      <div>
        <h2 className="text-lg font-bold text-text">Find your next home</h2>
        <p className="text-sm text-text-muted">
          Enter your email, we'll send you a 6-digit code. No password needed.
        </p>
      </div>

      <FormField label="Email address" required error={formError ?? undefined}>
        <Input
          type="email"
          placeholder="you@example.com"
          value={email}
          onChange={(event) => setEmail(event.target.value)}
          disabled={otpRequest.isPending || isCoolingDown}
          hasError={Boolean(formError)}
        />
      </FormField>

      {isCoolingDown && (
        <Alert variant="warning">
          Too many requests. Try again in {formatCountdown(secondsLeft)}.
        </Alert>
      )}

      {genericError && <Alert variant="error">{genericError}</Alert>}

      <Button type="submit" isLoading={otpRequest.isPending} disabled={isCoolingDown}>
        {isCoolingDown ? `Retry in ${formatCountdown(secondsLeft)}` : "Get the code"}
      </Button>

      <p className="text-center text-sm text-text-muted">
        By continuing, you agree to MyHouse's Terms of Service.
      </p>
    </form>
  );
}

export { OtpRequestForm };
export type { OtpRequestFormProps };
