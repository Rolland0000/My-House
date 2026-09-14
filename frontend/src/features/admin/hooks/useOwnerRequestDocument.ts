import { useEffect, useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { getOwnerRequestDocument, ownerRequestDocumentQueryKey } from "../api";

/**
 * Fetches one identity document on demand and exposes it as an object URL —
 * `enabled` stays false until the admin actually asks to view it, so a
 * detail page never downloads every document just by being opened.
 */
export function useOwnerRequestDocument(id: string, docId: string, enabled: boolean) {
  const query = useQuery({
    queryKey: ownerRequestDocumentQueryKey(id, docId),
    queryFn: () => getOwnerRequestDocument(id, docId),
    enabled,
    staleTime: Infinity,
  });

  const url = useMemo(() => (query.data ? URL.createObjectURL(query.data) : null), [query.data]);

  // Revoking only on cleanup (not via setState) avoids re-rendering just to
  // free the URL — the next render already has the new one from `useMemo`.
  useEffect(() => {
    return () => {
      if (url) URL.revokeObjectURL(url);
    };
  }, [url]);

  return { url, isPending: enabled && query.isPending, error: query.error };
}
