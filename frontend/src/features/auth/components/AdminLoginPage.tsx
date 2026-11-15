import { useState } from "react";
import type { FormEvent } from "react";
import { Navigate, useSearchParams } from "react-router";
import { Alert, Button, FormField, Input, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useCountdown } from "../../../shared/hooks/useCountdown";
import { formatCountdown } from "../../../shared/utils/format";
import { RETURN_TO_PARAM, safeAdminReturnTo } from "../../../shared/utils/returnTo";
import { ADMIN_LOCKED_FALLBACK_SECONDS } from "../constants";
import { useAdminLogin } from "../hooks/useAdminLogin";
import { useAuth } from "../hooks/useAuth";

const ADMIN_LOCKED_CODE = "ADMIN_LOGIN_LOCKED";

function errorMessage(error: unknown): string {
  if (!(error instanceof ApiError)) return "Something went wrong. Please try again.";
  if (error.code === "INVALID_CREDENTIALS") return "Incorrect email or password.";
  if (error.code === "ACCOUNT_SUSPENDED") return "This account has been suspended.";
  return error.message;
}

function AdminLoginPage() {
  const { status, sessionRole } = useAuth();
  const [searchParams] = useSearchParams();
  const returnTo = safeAdminReturnTo(searchParams.get(RETURN_TO_PARAM));
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [formError, setFormError] = useState<string | null>(null);
  const { secondsLeft, start } = useCountdown();
  const adminLogin = useAdminLogin();

  if (status === "bootstrapping") {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" />
      </div>
    );
  }

  // A signed-in non-admin stays on the form: sending them to /admin would only bounce them home.
  if (status === "authenticated" && sessionRole === "admin") {
    return <Navigate to={returnTo} replace />;
  }

  const isLocked = secondsLeft > 0;
  const apiError =
    adminLogin.error &&
    !(adminLogin.error instanceof ApiError && adminLogin.error.code === ADMIN_LOCKED_CODE)
      ? errorMessage(adminLogin.error)
      : null;

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!email.trim() || !password) {
      setFormError("Enter your email and password.");
      return;
    }
    setFormError(null);

    adminLogin.mutate(
      { email: email.trim(), password },
      {
        onError: (error) => {
          setPassword("");
          if (error instanceof ApiError && error.code === ADMIN_LOCKED_CODE) {
            start(error.retryAfterSeconds ?? ADMIN_LOCKED_FALLBACK_SECONDS);
          }
        },
      }
    );
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-5">
      <div>
        <h1 className="text-[22px] font-bold text-ink-900">Administration</h1>
        <p className="text-sm text-text-muted">Sign in with your administrator credentials.</p>
      </div>

      <FormField label="Email address" required>
        <Input
          type="email"
          autoComplete="username"
          value={email}
          onChange={(event) => setEmail(event.target.value)}
          disabled={adminLogin.isPending}
        />
      </FormField>

      <FormField label="Password" required error={formError ?? undefined}>
        <Input
          type="password"
          autoComplete="current-password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
          disabled={adminLogin.isPending}
          hasError={Boolean(formError)}
        />
      </FormField>

      {isLocked && (
        <Alert variant="warning">
          Too many failed attempts. Try again in {formatCountdown(secondsLeft)}.
        </Alert>
      )}

      {apiError && <Alert variant="error">{apiError}</Alert>}

      <Button type="submit" isLoading={adminLogin.isPending} disabled={isLocked}>
        {isLocked ? `Retry in ${formatCountdown(secondsLeft)}` : "Sign in"}
      </Button>
    </form>
  );
}

export { AdminLoginPage };
