import { useId, type ReactNode } from "react";
import { Trash2 } from "lucide-react";
import { Alert, Badge, Button, Spinner } from "../../../shared/components";
import type { ListingMedia } from "../api";
import { COVER_REQUIRED_MESSAGE } from "../listingPhotos";

interface TileFrameProps {
  imageUrl: string;
  alt: string;
  overlay?: ReactNode;
  actions: ReactNode;
  footer?: ReactNode;
}

function TileFrame({ imageUrl, alt, overlay, actions, footer }: TileFrameProps) {
  return (
    <li className="flex flex-col border border-border bg-surface">
      <div className="relative aspect-[4/3] overflow-hidden bg-primary-soft">
        <img src={imageUrl} alt={alt} className="size-full object-cover" />
        {overlay}
      </div>
      <div className="flex min-h-12 items-center justify-end gap-2 p-2">{actions}</div>
      {footer}
    </li>
  );
}

interface PhotoTileProps {
  photo: ListingMedia;
  alt: string;
  /** Another cover change or deletion is in flight. */
  actionsDisabled: boolean;
  isMakingCover: boolean;
  /** The cover can't be deleted while other photos remain. */
  isDeleteBlocked: boolean;
  onMakeCover: () => void;
  onDelete: () => void;
}

function PhotoTile({
  photo,
  alt,
  actionsDisabled,
  isMakingCover,
  isDeleteBlocked,
  onMakeCover,
  onDelete,
}: PhotoTileProps) {
  const hintId = useId();

  return (
    <TileFrame
      imageUrl={photo.url}
      alt={alt}
      actions={
        <>
          {photo.is_cover ? (
            <Badge tone="brass" className="mr-auto">
              Cover
            </Badge>
          ) : (
            <Button
              variant="secondary"
              size="sm"
              className="mr-auto"
              onClick={onMakeCover}
              disabled={actionsDisabled}
              isLoading={isMakingCover}
            >
              Make cover
            </Button>
          )}
          <Button
            variant="ghost"
            size="sm"
            onClick={onDelete}
            disabled={actionsDisabled || isDeleteBlocked}
            aria-label={`Delete ${alt}`}
            aria-describedby={isDeleteBlocked ? hintId : undefined}
          >
            <Trash2 className="size-4" aria-hidden="true" />
          </Button>
        </>
      }
      footer={
        isDeleteBlocked && (
          <p id={hintId} className="px-2 pb-2 text-xs text-text-muted">
            {COVER_REQUIRED_MESSAGE}
          </p>
        )
      }
    />
  );
}

interface PendingPhotoTileProps {
  fileName: string;
  previewUrl: string;
  error: string | null;
  onRemove: () => void;
}

function PendingPhotoTile({ fileName, previewUrl, error, onRemove }: PendingPhotoTileProps) {
  return (
    <TileFrame
      imageUrl={previewUrl}
      alt={`Preview of ${fileName}`}
      overlay={
        !error && (
          <div className="absolute inset-0 flex items-center justify-center bg-surface/60">
            <Spinner size="lg" label={`Uploading ${fileName}…`} />
          </div>
        )
      }
      actions={
        <>
          <span className="mr-auto truncate text-sm text-text-muted">{fileName}</span>
          {error && (
            <Button variant="secondary" size="sm" onClick={onRemove}>
              Remove
            </Button>
          )}
        </>
      }
      footer={
        error && (
          <Alert variant="error" className="m-2 mt-0">
            {error}
          </Alert>
        )
      }
    />
  );
}

export { PhotoTile, PendingPhotoTile };
export type { PhotoTileProps, PendingPhotoTileProps };
