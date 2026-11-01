import type { QueryClient } from "@tanstack/react-query";
import { apiGet } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

type ContactResponse = components["schemas"]["ContactResponse"];

const CONTACT_QUERY_ROOT = "listing-contact";

export function listingContactQueryKey(listingId: string) {
  return [CONTACT_QUERY_ROOT, listingId] as const;
}

export function getListingContact(listingId: string): Promise<ContactResponse> {
  return apiGet<ContactResponse>(`/api/v1/listings/${encodeURIComponent(listingId)}/contact`);
}

/** Called on sign-out, so the next user of the tab never sees the numbers the previous one revealed. */
export function removeContactQueries(queryClient: QueryClient) {
  queryClient.removeQueries({ queryKey: [CONTACT_QUERY_ROOT] });
}
