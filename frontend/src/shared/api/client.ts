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

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, {
    ...init,
    credentials: "include",
    headers: { Accept: "application/json", ...init?.headers },
  });

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
