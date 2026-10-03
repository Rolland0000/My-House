import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { setAccessTokenGetter, setUnauthorizedHandler } from "../../shared/api/client";
import { refreshSession } from "./api";
import { readTokenRole } from "./tokenRole";

type AuthStatus = "bootstrapping" | "authenticated" | "anonymous";
type SessionRole = ReturnType<typeof readTokenRole>;

interface AuthContextValue {
  status: AuthStatus;
  /** Role claimed by the current access token; may lag behind the role stored in DB. */
  sessionRole: SessionRole;
  setSession: (accessToken: string) => void;
  clearSession: () => void;
  refresh: () => Promise<string | null>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

function AuthProvider({ children }: { children: ReactNode }) {
  const [status, setStatus] = useState<AuthStatus>("bootstrapping");
  const [sessionRole, setSessionRole] = useState<SessionRole>(null);
  const tokenRef = useRef<string | null>(null);
  const refreshInFlight = useRef<Promise<string | null> | null>(null);

  const clearSession = useCallback(() => {
    tokenRef.current = null;
    setSessionRole(null);
    setStatus("anonymous");
  }, []);

  const setSession = useCallback((accessToken: string) => {
    tokenRef.current = accessToken;
    setSessionRole(readTokenRole(accessToken));
    setStatus("authenticated");
  }, []);

  const refresh = useCallback((): Promise<string | null> => {
    if (!refreshInFlight.current) {
      refreshInFlight.current = refreshSession()
        .then(({ data }) => {
          setSession(data.access_token);
          return data.access_token;
        })
        .catch(() => {
          clearSession();
          return null;
        })
        .finally(() => {
          refreshInFlight.current = null;
        });
    }
    return refreshInFlight.current;
  }, [setSession, clearSession]);

  useEffect(() => {
    setAccessTokenGetter(() => tokenRef.current);
    setUnauthorizedHandler(refresh);
    return () => setUnauthorizedHandler(null);
  }, [refresh]);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const value = useMemo(
    () => ({ status, sessionRole, setSession, clearSession, refresh }),
    [status, sessionRole, setSession, clearSession, refresh]
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

function useAuth(): AuthContextValue {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}

export { AuthProvider, useAuth };
export type { AuthStatus, AuthContextValue };
