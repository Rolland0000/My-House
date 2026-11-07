import type { QueryClient } from "@tanstack/react-query";
import { apiDelete, apiGet, apiGetBlob, apiPatch } from "../../shared/api/client";
import type { components, paths } from "../../shared/api/types";
import type { ListingStatus } from "../listings";

export type AdminOwnerRequest = components["schemas"]["AdminOwnerRequestDto"];
export type AdminOwnerRequestDetail = components["schemas"]["AdminOwnerRequestDetailDto"];
export type AdminOwnerRequestDocument = components["schemas"]["AdminOwnerRequestDocumentDto"];
export type ReviewDecision = components["schemas"]["ReviewDecision"];
export type AdminListing = components["schemas"]["AdminListingDto"];
export type AdminListingVisibility = components["schemas"]["AdminListingVisibility"];
export type AdminUser = components["schemas"]["AdminUserDto"];
export type AdminUsersResult = components["schemas"]["PaginatedResponse_AdminUserDto"];
export type UserRole = components["schemas"]["Role"];
export type ListAdminUsersParams = NonNullable<
  paths["/api/v1/admin/users"]["get"]["parameters"]["query"]
>;
export type AccountState = NonNullable<ListAdminUsersParams["state"]>;

type OwnerRequestQueueResponse = components["schemas"]["PaginatedResponse_AdminOwnerRequestDto"];
type AdminOwnerRequestDetailResponse = components["schemas"]["AdminOwnerRequestDetailResponse"];
type AdminListingsResponse = components["schemas"]["PaginatedResponse_AdminListingDto"];
type AdminListingResponse = components["schemas"]["AdminListingResponse"];

export interface ListOwnerRequestsParams {
  status?: string;
  page?: number;
  per_page?: number;
}

export interface ListAdminListingsParams {
  status?: ListingStatus;
  visibility?: AdminListingVisibility;
  owner_id?: string;
  page?: number;
}

export const ownerRequestQueueQueryKey = (params: ListOwnerRequestsParams) =>
  ["admin", "owner-requests", params] as const;

export const ownerRequestDetailQueryKey = (id: string) => ["admin", "owner-requests", id] as const;

export const ownerRequestDocumentQueryKey = (id: string, docId: string) =>
  ["admin", "owner-requests", id, "documents", docId] as const;

export const ADMIN_QUERY_ROOT = "admin";
export const ADMIN_LISTINGS_KEY = [ADMIN_QUERY_ROOT, "listings"] as const;

export const adminListingsQueryKey = (params: ListAdminListingsParams) =>
  [...ADMIN_LISTINGS_KEY, params] as const;

/** Shared by a row's status change and delete, so either one locks both. */
export const adminListingActionKey = (id: string) => ["admin-listing-action", id] as const;

export const ADMIN_USERS_KEY = [ADMIN_QUERY_ROOT, "users"] as const;

export const adminUsersQueryKey = (params: ListAdminUsersParams) =>
  [...ADMIN_USERS_KEY, params] as const;

/** Lets a row lock its actions while a mutation on that account runs. */
export const adminUserMutationKey = (id: string) => ["admin-user", id] as const;

/** Called on sign-out from either layout: the cached lists hold every account's email and phone. */
export function removeAdminQueries(queryClient: QueryClient) {
  queryClient.removeQueries({ queryKey: [ADMIN_QUERY_ROOT] });
}

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

export function reviewOwnerRequest(
  id: string,
  status: ReviewDecision,
  adminNote?: string
): Promise<AdminOwnerRequestDetailResponse> {
  return apiPatch(`/api/v1/admin/owner-requests/${id}`, { status, admin_note: adminNote });
}

export function listAdminListings(params: ListAdminListingsParams): Promise<AdminListingsResponse> {
  return apiGet<AdminListingsResponse>("/api/v1/admin/listings", {
    status: params.status,
    visibility: params.visibility,
    owner_id: params.owner_id,
    page: params.page,
  });
}

export function updateAdminListingStatus(
  id: string,
  status: ListingStatus
): Promise<AdminListingResponse> {
  return apiPatch(`/api/v1/admin/listings/${id}`, { status });
}

export function deleteAdminListing(id: string): Promise<void> {
  return apiDelete(`/api/v1/admin/listings/${id}`);
}

export function listAdminUsers(params: ListAdminUsersParams): Promise<AdminUsersResult> {
  return apiGet<AdminUsersResult>("/api/v1/admin/users", params);
}

export function deleteAdminUser(id: string): Promise<void> {
  return apiDelete<void>(`/api/v1/admin/users/${encodeURIComponent(id)}`);
}
