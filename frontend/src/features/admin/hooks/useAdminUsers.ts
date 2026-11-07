import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { adminUsersQueryKey, listAdminUsers, type ListAdminUsersParams } from "../api";

export function useAdminUsers(params: ListAdminUsersParams) {
  return useQuery({
    queryKey: adminUsersQueryKey(params),
    queryFn: () => listAdminUsers(params),
    placeholderData: keepPreviousData,
  });
}
