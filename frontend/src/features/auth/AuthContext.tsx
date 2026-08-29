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

type AuthStatus = "bootstrapping" | "authenticated" | "anonymous";

interface AuthContextValue {
  status: AuthStatus;
  setSession: (accessToken: string) => void;
  clearSession: () => void;
  refresh: () => Promise<string | null>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

function AuthProvider({ children }: { children: ReactNode }) {
  const [status, setStatus] = useState<AuthStatus>("bootstrapping");
  const tokenRef = useRef<string | null>(null);
  const refreshInFlight = useRef<Promise<string | null> | null>(null);

  const clearSession = useCallback(() => {
    tokenRef.current = null;
    setStatus("anonymous");
  }, []);

  const setSession = useCallback((accessToken: string) => {
    tokenRef.current = accessToken;
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
    () => ({ status, setSession, clearSession, refresh }),
    [status, setSession, clearSession, refresh]
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
