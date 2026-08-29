import { useState } from "react";
import { Navigate, useNavigate } from "react-router";
import { Spinner } from "../../../shared/components";
import { useAuth } from "../hooks/useAuth";
import { OtpRequestForm } from "./OtpRequestForm";
import { OtpVerifyForm } from "./OtpVerifyForm";
import { ProfileSetupForm } from "./ProfileSetupForm";

type Screen = "email" | "code" | "profile";

function AuthFlow() {
  const { status } = useAuth();
  const navigate = useNavigate();
  const [screen, setScreen] = useState<Screen>("email");
  const [email, setEmail] = useState("");

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
        <OtpRequestForm
          initialEmail={email}
          onRequested={(requestedEmail) => {
            setEmail(requestedEmail);
            setScreen("code");
          }}
        />
      );
    case "code":
      return (
        <OtpVerifyForm
          email={email}
          onBack={() => setScreen("email")}
          onVerified={(isNewUser) => {
            if (isNewUser) {
              setScreen("profile");
            } else {
              navigate("/", { replace: true });
            }
          }}
        />
      );
    case "profile":
      return <ProfileSetupForm email={email} onDone={() => navigate("/", { replace: true })} />;
  }
}

export { AuthFlow };
