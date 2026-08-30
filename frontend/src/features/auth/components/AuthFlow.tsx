import { useState } from "react";
import { Navigate, useNavigate } from "react-router";
import { Alert, Spinner } from "../../../shared/components";
import { useAuth } from "../hooks/useAuth";
import { OtpRequestForm } from "./OtpRequestForm";
import { OtpVerifyForm } from "./OtpVerifyForm";
import { RegistrationForm } from "./RegistrationForm";

type Screen = "email" | "code" | "register";

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
        <div className="flex flex-col gap-5">
          {wasInterrupted && (
            <Alert variant="info">
              Votre inscription a été interrompue et aucun compte n'a été créé. Demandez un nouveau
              code pour reprendre.
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
      );
    case "code":
      return (
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
      );
    case "register":
      return (
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
      );
  }
}

export { AuthFlow };
