import { apiGet, apiGetBlob } from "../../shared/api/client";
import type { components } from "../../shared/api/types";

export type AdminOwnerRequest = components["schemas"]["AdminOwnerRequestDto"];
export type AdminOwnerRequestDetail = components["schemas"]["AdminOwnerRequestDetailDto"];
export type AdminOwnerRequestDocument = components["schemas"]["AdminOwnerRequestDocumentDto"];

type OwnerRequestQueueResponse = components["schemas"]["PaginatedResponse_AdminOwnerRequestDto"];
type AdminOwnerRequestDetailResponse = components["schemas"]["AdminOwnerRequestDetailResponse"];

export interface ListOwnerRequestsParams {
  status?: string;
  page?: number;
  per_page?: number;
}

export const ownerRequestQueueQueryKey = (params: ListOwnerRequestsParams) =>
  ["admin", "owner-requests", params] as const;

export const ownerRequestDetailQueryKey = (id: string) => ["admin", "owner-requests", id] as const;

export const ownerRequestDocumentQueryKey = (id: string, docId: string) =>
  ["admin", "owner-requests", id, "documents", docId] as const;

export function listOwnerRequests(
  params: ListOwnerRequestsParams
): Promise<OwnerRequestQueueResponse> {
  return apiGet<OwnerRequestQueueResponse>("/api/v1/admin/owner-requests", {
    status: params.status,
    page: params.page,
    per_page: params.per_page,
  });
}

export function getOwnerRequest(id: string): Promise<AdminOwnerRequestDetailResponse> {
  return apiGet(`/api/v1/admin/owner-requests/${id}`);
}

export function getOwnerRequestDocument(id: string, docId: string): Promise<Blob> {
  return apiGetBlob(`/api/v1/admin/owner-requests/${id}/documents/${docId}`);
}
