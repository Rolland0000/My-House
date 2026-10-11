import { useEffect, useRef, useState } from "react";
import { Link, useNavigate, useParams } from "react-router";
import { useIsMutating } from "@tanstack/react-query";
import { ArrowLeft } from "lucide-react";
import { Alert, Button, FileDropzone, Spinner, useToast } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { ACCEPTED_IMAGE_TYPES, MAX_PHOTOS_PER_LISTING } from "../../../shared/api/constants";
import { isRemoteMediaUrl } from "../../../shared/utils/mediaUrl";
import { useProfile } from "../../profile";
import { useListing } from "../hooks/useListings";
import {
  LISTING_PHOTOS_MUTATION_KEY,
  useDeleteListingPhoto,
  useReorderListingPhotos,
  useSetListingCover,
  useUploadListingPhoto,
} from "../hooks/useListingPhotoMutations";
import {
  MAX_IMAGE_SIZE_MB,
  PROBLEM_MESSAGES,
  freePhotoSlots,
  photoUploadErrorMessage,
  planPhotoSelection,
} from "../listingPhotos";
import { moveControl, movePhoto, type MoveDirection } from "../photoOrder";
import { DeletePhotoModal } from "./DeletePhotoModal";
import { ListingNotFound } from "./ListingNotFound";
import { PendingPhotoTile, PhotoTile } from "./PhotoTile";

interface PendingUpload {
  key: string;
  file: File;
  previewUrl: string;
  error: string | null;
}

interface MoveFocusTarget {
  mediaId: string;
  /** Index the photo lands on, so focus waits for the optimistic order to render. */
  index: number;
  direction: MoveDirection;
}

interface SelectionNotice {
  rejected: { name: string; message: string }[];
  skippedCount: number;
}

function isNotFound(error: unknown) {
  return error instanceof ApiError && error.status === 404;
}

function skippedMessage(count: number) {
  const files = count === 1 ? "1 file was" : `${count} files were`;
  return `${files} not uploaded: a listing can have ${MAX_PHOTOS_PER_LISTING} photos at most.`;
}

function ListingPhotosPage() {
  const { id = "" } = useParams<"id">();
  const navigate = useNavigate();
  const { showToast } = useToast();
  const { data, isPending, error } = useListing(id);
  const { data: profile, isPending: isProfilePending } = useProfile();
  const upload = useUploadListingPhoto(id);
  const coverChange = useSetListingCover(id);
  const deletion = useDeleteListingPhoto(id);
  const reorder = useReorderListingPhotos(id);
  const isMutatingPhotos = useIsMutating({ mutationKey: [...LISTING_PHOTOS_MUTATION_KEY, id] }) > 0;

  const [pendingUploads, setPendingUploads] = useState<PendingUpload[]>([]);
  const [selectionNotice, setSelectionNotice] = useState<SelectionNotice | null>(null);
  const [photoToDelete, setPhotoToDelete] = useState<string | null>(null);
  const nextUploadKey = useRef(0);
  const previewUrls = useRef(new Set<string>());
  const [moveAnnouncement, setMoveAnnouncement] = useState({ id: 0, text: "" });
  const moveFocusTarget = useRef<MoveFocusTarget | null>(null);
  const gridRef = useRef<HTMLUListElement>(null);

  // Frees every preview still alive when the screen unmounts, uploads in flight included.
  useEffect(() => {
    const urls = previewUrls.current;
    return () => urls.forEach((url) => URL.revokeObjectURL(url));
  }, []);

  // React may re-insert the moved tile, which drops keyboard focus; put it back on a usable button.
  useEffect(() => {
    const target = moveFocusTarget.current;
    if (!target || data?.data.media[target.index]?.id !== target.mediaId) return;
    moveFocusTarget.current = null;
    gridRef.current
      ?.querySelector<HTMLButtonElement>(
        `[data-move-control="${moveControl(target.mediaId, target.direction)}"]`
      )
      ?.focus();
  });

  if (isPending || isProfilePending) {
    return (
      <div className="flex justify-center py-24">
        <Spinner size="lg" label="Loading listing…" />
      </div>
    );
  }

  if (isNotFound(error)) {
    return <ListingNotFound />;
  }

  // A failed refetch keeps the cached data, so only a first load that failed blocks the
  // screen; replacing it would hide the uploads in flight.
  if (!data || !profile) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        <Alert variant="error" title="Unable to load this listing">
          Please try again in a moment.
        </Alert>
      </div>
    );
  }

  const listing = data.data;

  // A published listing is readable by anyone, so ownership is checked here too;
  // the photo endpoints still answer 404 for someone else's listing.
  if (profile.id !== listing.owner.id) {
    return <ListingNotFound />;
  }

  const photoCount = listing.media.length;
  const displayablePhotos = listing.media.filter((photo) => isRemoteMediaUrl(photo.url));
  const inFlightCount = pendingUploads.filter((pending) => !pending.error).length;
  const freeSlots = freePhotoSlots(photoCount, inFlightCount);
  // No cover change or deletion during an order save: rolling back a failed save would undo
  // the cover change, and a deletion makes the save fail with 409.
  const isPhotoActionPending = coverChange.isPending || deletion.isPending || reorder.isPending;
  const isMoveLocked = isMutatingPhotos || inFlightCount > 0;

  function dropPendingUpload(pending: PendingUpload) {
    URL.revokeObjectURL(pending.previewUrl);
    previewUrls.current.delete(pending.previewUrl);
    setPendingUploads((current) => current.filter(({ key }) => key !== pending.key));
  }

  function startUpload(pending: PendingUpload) {
    upload.mutateAsync(pending.file).then(
      () => dropPendingUpload(pending),
      (uploadError: unknown) =>
        setPendingUploads((current) =>
          current.map((item) =>
            item.key === pending.key
              ? { ...item, error: photoUploadErrorMessage(uploadError) }
              : item
          )
        )
    );
  }

  function handleFilesSelected(files: File[]) {
    const plan = planPhotoSelection(files, freeSlots);
    const hasNotice = plan.rejected.length > 0 || plan.skippedCount > 0;
    setSelectionNotice(
      hasNotice
        ? {
            rejected: plan.rejected.map(({ file, problem }) => ({
              name: file.name,
              message: PROBLEM_MESSAGES[problem],
            })),
            skippedCount: plan.skippedCount,
          }
        : null
    );

    const uploads = plan.toUpload.map((file) => {
      const previewUrl = URL.createObjectURL(file);
      previewUrls.current.add(previewUrl);
      return { key: `upload-${nextUploadKey.current++}`, file, previewUrl, error: null };
    });
    setPendingUploads((current) => [...current, ...uploads]);
    uploads.forEach(startUpload);
  }

  function handleMakeCover(mediaId: string) {
    coverChange.mutate(mediaId, {
      onError: () =>
        showToast("Couldn't change the cover. Please try again.", { variant: "error" }),
    });
  }

  // The order sent covers every photo, hidden ones included: the server answers 409 for a partial list.
  function handleMove(mediaId: string, direction: MoveDirection) {
    const reordered = movePhoto(listing.media, mediaId, direction);
    if (!reordered) return;
    const index = reordered.findIndex((photo) => photo.id === mediaId);
    const canMoveAgain = movePhoto(reordered, mediaId, direction) !== null;
    moveFocusTarget.current = {
      mediaId,
      index,
      direction: canMoveAgain ? direction : direction === 1 ? -1 : 1,
    };
    const visibleOrder = reordered.filter((photo) => isRemoteMediaUrl(photo.url));
    const visibleIndex = visibleOrder.findIndex((photo) => photo.id === mediaId);
    setMoveAnnouncement((previous) => ({
      id: previous.id + 1,
      text: `Photo moved to position ${visibleIndex + 1} of ${visibleOrder.length}.`,
    }));
    reorder.mutate(reordered, {
      onSettled: () => {
        moveFocusTarget.current = null;
      },
    });
  }

  function openDeleteModal(mediaId: string) {
    deletion.reset();
    setPhotoToDelete(mediaId);
  }

  function confirmDelete() {
    if (!photoToDelete) return;
    deletion.mutate(photoToDelete, { onSuccess: () => setPhotoToDelete(null) });
  }

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-6 px-6 py-6">
      <Link
        to={`/listings/${id}`}
        className="flex w-fit items-center gap-1 text-sm font-semibold text-ink-500 hover:text-ink-900"
      >
        <ArrowLeft className="size-4" aria-hidden="true" />
        Back to listing
      </Link>

      <div>
        <h1 className="text-2xl font-bold text-ink-900">Photos</h1>
        <p className="mt-1 text-sm font-medium break-words text-ink-500">{listing.title}</p>
      </div>

      {photoCount === 0 && pendingUploads.length === 0 && (
        <p className="text-sm text-text-muted">
          No photos yet. The first photo you add becomes the cover.
        </p>
      )}

      {(displayablePhotos.length > 0 || pendingUploads.length > 0) && (
        <ul ref={gridRef} className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {displayablePhotos.map((photo, index) => (
            <PhotoTile
              key={photo.id}
              photo={photo}
              alt={`Photo ${index + 1} of ${listing.title}`}
              actionsDisabled={isPhotoActionPending}
              isMakingCover={coverChange.isPending && coverChange.variables === photo.id}
              isDeleteBlocked={photo.is_cover && photoCount > 1}
              onMakeCover={() => handleMakeCover(photo.id)}
              onDelete={() => openDeleteModal(photo.id)}
              displayPosition={index + 1}
              canMoveEarlier={movePhoto(listing.media, photo.id, -1) !== null}
              canMoveLater={movePhoto(listing.media, photo.id, 1) !== null}
              isMoveLocked={isMoveLocked}
              onMoveEarlier={() => handleMove(photo.id, -1)}
              onMoveLater={() => handleMove(photo.id, 1)}
            />
          ))}
          {pendingUploads.map((pending) => (
            <PendingPhotoTile
              key={pending.key}
              fileName={pending.file.name}
              previewUrl={pending.previewUrl}
              error={pending.error}
              onRemove={() => dropPendingUpload(pending)}
            />
          ))}
        </ul>
      )}

      {freeSlots > 0 ? (
        <FileDropzone
          accept={ACCEPTED_IMAGE_TYPES.join(",")}
          multiple
          onFilesSelected={handleFilesSelected}
          label="Drag photos here, or click to choose them"
          hint={`JPEG, PNG, or WebP · ${MAX_IMAGE_SIZE_MB} MB max per photo · ${MAX_PHOTOS_PER_LISTING} photos max`}
        />
      ) : (
        <Alert variant="info">
          You&apos;ve reached the {MAX_PHOTOS_PER_LISTING}-photo limit.
          {photoCount >= MAX_PHOTOS_PER_LISTING && " Delete a photo to add another."}
        </Alert>
      )}

      {selectionNotice && (
        <Alert variant="warning">
          <ul className="flex flex-col gap-1">
            {selectionNotice.rejected.map(({ name, message }, index) => (
              <li key={`${name}-${index}`} className="break-words">
                <span className="font-semibold">{name}</span>: {message}
              </li>
            ))}
            {selectionNotice.skippedCount > 0 && (
              <li>{skippedMessage(selectionNotice.skippedCount)}</li>
            )}
          </ul>
        </Alert>
      )}

      {/* A new key re-inserts the text, so the same message is announced again. */}
      <p role="status" className="sr-only">
        <span key={moveAnnouncement.id}>{moveAnnouncement.text}</span>
      </p>

      <div className="flex justify-end">
        <Button onClick={() => navigate(`/listings/${id}`)} disabled={isMutatingPhotos}>
          Finish
        </Button>
      </div>

      <DeletePhotoModal
        isOpen={photoToDelete !== null}
        isPending={deletion.isPending}
        error={deletion.error}
        onConfirm={confirmDelete}
        onClose={() => setPhotoToDelete(null)}
      />
    </div>
  );
}

export { ListingPhotosPage };
