import { useState } from "react";
import { Navigate, useNavigate } from "react-router";
import { Check } from "lucide-react";
import { Alert, Spinner } from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";
import { useAuth } from "../hooks/useAuth";
import { OtpRequestForm } from "./OtpRequestForm";
import { OtpVerifyForm } from "./OtpVerifyForm";
import { RegistrationForm } from "./RegistrationForm";

type Screen = "email" | "code" | "register";

const STEP_ORDER: Screen[] = ["email", "code", "register"];

/** Only three real steps exist and they happen in order, so numbering them here
 *  is honest rather than decorative — see the "Plan & Brass" auth mockups. */
function AuthStepIndicator({ screen }: { screen: Screen }) {
  const currentIndex = STEP_ORDER.indexOf(screen);

  return (
    <div className="mb-6 flex items-center gap-0" aria-hidden="true">
      {STEP_ORDER.map((step, index) => (
        <div key={step} className="flex flex-1 items-center last:flex-none">
          <span
            className={cn(
              "flex size-5.5 flex-none items-center justify-center text-xs font-bold",
              index < currentIndex && "bg-success text-white",
              index === currentIndex && "bg-ink-900 text-white",
              index > currentIndex && "border border-border-strong bg-surface text-ink-500"
            )}
          >
            {index < currentIndex ? <Check className="size-3" /> : index + 1}
          </span>
          {index < STEP_ORDER.length - 1 && <span className="h-px flex-1 bg-border-strong" />}
        </div>
      ))}
    </div>
  );
}

// The ticket itself never leaves React state (it proves email possession, like
// an access token). Only this marker survives a reload, so the email screen can
// explain why the user landed back on it.
const INTERRUPTED_KEY = "mh.auth.registration_interrupted";

function readInterrupted(): boolean {
  try {
    return sessionStorage.getItem(INTERRUPTED_KEY) !== null;
  } catch {
    return false;
  }
}

function markInterrupted(pending: boolean): void {
  try {
    if (pending) sessionStorage.setItem(INTERRUPTED_KEY, "1");
    else sessionStorage.removeItem(INTERRUPTED_KEY);
  } catch {
    // Private browsing / blocked storage — the flow works without the notice.
  }
}

function AuthFlow() {
  const { status } = useAuth();
  const navigate = useNavigate();
  const [screen, setScreen] = useState<Screen>("email");
  const [email, setEmail] = useState("");
  const [registrationTicket, setRegistrationTicket] = useState("");
  const [wasInterrupted, setWasInterrupted] = useState(readInterrupted);

  function leaveRegistration() {
    markInterrupted(false);
    setWasInterrupted(false);
    setRegistrationTicket("");
  }

  if (status === "bootstrapping") {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" />
      </div>
    );
  }

  if (status === "authenticated" && screen === "email") {
    return <Navigate to="/" replace />;
  }

  switch (screen) {
    case "email":
      return (
        <div>
          <AuthStepIndicator screen={screen} />
          <div className="flex flex-col gap-5">
            {wasInterrupted && (
              <Alert variant="info">
                Your registration was interrupted and no account was created. Request a new code to
                continue.
              </Alert>
            )}
            <OtpRequestForm
              initialEmail={email}
              onRequested={(requestedEmail) => {
                leaveRegistration();
                setEmail(requestedEmail);
                setScreen("code");
              }}
            />
          </div>
        </div>
      );
    case "code":
      return (
        <div>
          <AuthStepIndicator screen={screen} />
          <OtpVerifyForm
            email={email}
            onBack={() => setScreen("email")}
            onVerified={(ticket) => {
              if (ticket) {
                markInterrupted(true);
                setRegistrationTicket(ticket);
                setScreen("register");
              } else {
                navigate("/", { replace: true });
              }
            }}
          />
        </div>
      );
    case "register":
      return (
        <div>
          <AuthStepIndicator screen={screen} />
          <RegistrationForm
            email={email}
            registrationTicket={registrationTicket}
            onRegistered={() => {
              markInterrupted(false);
              navigate("/", { replace: true });
            }}
            onAccountExists={() => {
              leaveRegistration();
              setScreen("email");
            }}
          />
        </div>
      );
  }
}

export { AuthFlow };
