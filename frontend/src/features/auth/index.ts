export { AuthProvider } from "./AuthContext";
export type { AuthStatus, AuthContextValue } from "./AuthContext";

// AuthFlow is deliberately not re-exported: router.tsx lazy-loads it by path,
// and a static re-export here would pull it into the eager bundle.

export { useAuth } from "./hooks/useAuth";
export { useOtpRequest } from "./hooks/useOtpRequest";
export { useOtpVerify } from "./hooks/useOtpVerify";
export { useRegister } from "./hooks/useRegister";

export {
  requestOtp,
  verifyOtp,
  registerAccount,
  refreshSession,
  logout,
  type OtpVerifyToken,
  type RegisterPayload,
} from "./api";
