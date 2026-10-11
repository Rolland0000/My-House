import { useMutation, useQueryClient, type QueryClient } from "@tanstack/react-query";
import {
  deleteListingPhoto,
  setListingCover,
  uploadListingPhoto,
  type ListingDetail,
  type ListingMedia,
} from "../api";
import { isAlreadyDeleted } from "../listingDeletion";
import { LISTING_STATUS_MUTATION_KEY } from "./useUpdateListingStatus";

export const LISTING_PHOTOS_MUTATION_KEY = ["listing-photos"];

function updateListingMedia(
  queryClient: QueryClient,
  listingId: string,
  update: (media: ListingMedia[]) => ListingMedia[]
) {
  queryClient.setQueryData<{ data: ListingDetail }>(["listing", listingId], (response) =>
    response ? { data: { ...response.data, media: update(response.data.media) } } : response
  );
}

/** The settling mutation still counts as pending, so only the listing's last photo change refetches. */
function refetchAfterLastPhotoChange(queryClient: QueryClient, listingId: string) {
  const mutationKey = [...LISTING_PHOTOS_MUTATION_KEY, listingId];
  if (queryClient.isMutating({ mutationKey }) !== 1) return;
  void queryClient.invalidateQueries({ queryKey: ["listing", listingId] });
  void queryClient.invalidateQueries({ queryKey: ["listings"] });
  // A pending status change refetches the list when it settles; refetching now
  // would overwrite its optimistic state.
  if (queryClient.isMutating({ mutationKey: LISTING_STATUS_MUTATION_KEY }) === 0) {
    void queryClient.invalidateQueries({ queryKey: ["owner-listings"] });
  }
}

/** Call through `mutateAsync`: `mutate()` callbacks only fire for the latest of concurrent uploads. */
export function useUploadListingPhoto(listingId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [...LISTING_PHOTOS_MUTATION_KEY, listingId],
    mutationFn: (file: File) => uploadListingPhoto(listingId, file),
    retry: false,
    // A window-focus refetch may already include this photo.
    onSuccess: (response) =>
      updateListingMedia(queryClient, listingId, (media) =>
        media.some(({ id }) => id === response.data.id) ? media : [...media, response.data]
      ),
    onSettled: () => refetchAfterLastPhotoChange(queryClient, listingId),
  });
}

export function useDeleteListingPhoto(listingId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [...LISTING_PHOTOS_MUTATION_KEY, listingId],
    mutationFn: async (mediaId: string) => {
      try {
        await deleteListingPhoto(mediaId);
      } catch (error) {
        if (!isAlreadyDeleted(error)) throw error;
      }
    },
    retry: false,
    onSuccess: (_, mediaId) =>
      updateListingMedia(queryClient, listingId, (media) =>
        media.filter(({ id }) => id !== mediaId)
      ),
    onSettled: () => refetchAfterLastPhotoChange(queryClient, listingId),
  });
}

export function useSetListingCover(listingId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [...LISTING_PHOTOS_MUTATION_KEY, listingId],
    mutationFn: (mediaId: string) => setListingCover(listingId, mediaId),
    retry: false,
    onSuccess: (response) =>
      updateListingMedia(queryClient, listingId, (media) =>
        media.map((photo) => ({ ...photo, is_cover: photo.id === response.data.id }))
      ),
    onSettled: () => refetchAfterLastPhotoChange(queryClient, listingId),
  });
}
