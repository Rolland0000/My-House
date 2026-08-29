// Thin fetch wrapper — the ONLY place in the app that calls `fetch` directly.
// Every feature's `api.ts` goes through this (see .claude/rules/react-typecrypt.md).
//
// Requests are same-origin (relative paths): the Vite dev proxy and prod
// nginx both forward `/api/*` to the backend, so no base URL or CORS
// handling is needed here.

interface ErrorEnvelope {
  error: {
    code: string;
    message: string;
    status: number;
  };
}

/** Mirrors the backend's `AppError` JSON envelope (`shared/errors.rs`). */
export class ApiError extends Error {
  readonly status: number;
  readonly code: string;

  constructor(status: number, code: string, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.code = code;
  }
}

type AccessTokenGetter = () => string | null;
type UnauthorizedHandler = () => Promise<string | null>;

let readAccessToken: AccessTokenGetter = () => null;
let handleUnauthorized: UnauthorizedHandler | null = null;

// Registered once by AuthProvider. Going through callbacks rather than an
// import keeps the access token out of this module's own state — and out of
// any storage — without making the client depend on React.
export function setAccessTokenGetter(getter: AccessTokenGetter): void {
  readAccessToken = getter;
}

export function setUnauthorizedHandler(handler: UnauthorizedHandler | null): void {
  handleUnauthorized = handler;
}

/** `/auth/*` answers 401 on its own terms — refreshing there would loop. */
function isAuthPath(path: string): boolean {
  return path.startsWith("/api/v1/auth/");
}

function send(path: string, init?: RequestInit): Promise<Response> {
  const token = readAccessToken();
  return fetch(path, {
    ...init,
    credentials: "include",
    headers: {
      Accept: "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...init?.headers,
    },
  });
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  let response = await send(path, init);

  if (response.status === 401 && handleUnauthorized && !isAuthPath(path)) {
    const refreshedToken = await handleUnauthorized();
    if (refreshedToken) {
      response = await send(path, init);
    }
  }

  if (!response.ok) {
    const body: Partial<ErrorEnvelope> | null = await response.json().catch(() => null);
    throw new ApiError(
      response.status,
      body?.error?.code ?? "UNKNOWN_ERROR",
      body?.error?.message ?? response.statusText
    );
  }

  if (response.status === 204) {
    return undefined as T;
  }
  return (await response.json()) as T;
}

type QueryValue = string | number | boolean | undefined;

function buildQueryString(params?: Record<string, QueryValue>): string {
  if (!params) return "";
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined) search.set(key, String(value));
  }
  const query = search.toString();
  return query ? `?${query}` : "";
}

export function apiGet<T>(path: string, params?: Record<string, QueryValue>): Promise<T> {
  return request<T>(`${path}${buildQueryString(params)}`);
}

// The body is serialized here, never streamed, so a request can be replayed
// as-is after a token refresh.
function jsonInit(method: string, body?: unknown): RequestInit {
  if (body === undefined) return { method };
  return {
    method,
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  };
}

export function apiPost<T>(path: string, body?: unknown): Promise<T> {
  return request<T>(path, jsonInit("POST", body));
}

export function apiPut<T>(path: string, body?: unknown): Promise<T> {
  return request<T>(path, jsonInit("PUT", body));
}
