import { useEffect, useState } from "react";
import { UserRound } from "lucide-react";
import { Alert, Button, FileDropzone, useToast } from "../../../shared/components";
import { ACCEPTED_AVATAR_TYPES } from "../../../shared/api/constants";
import { isDisplayableMediaUrl } from "../../../shared/utils/mediaUrl";
import { preCheck, serverMessage } from "../avatarValidation";
import { useUploadAvatar } from "../hooks/useUploadAvatar";

interface AvatarUploadProps {
  avatarUrl: string | null;
}

function AvatarUpload({ avatarUrl }: AvatarUploadProps) {
  const [file, setFile] = useState<File | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [brokenUrl, setBrokenUrl] = useState<string | null>(null);
  const { showToast } = useToast();
  const upload = useUploadAvatar();

  // Frees the object URL when it is replaced and when the component unmounts.
  useEffect(() => {
    if (!previewUrl) return;
    return () => URL.revokeObjectURL(previewUrl);
  }, [previewUrl]);

  function clearSelection() {
    setFile(null);
    setPreviewUrl(null);
  }

  function handleFilesSelected(files: File[]) {
    const selected = files[0];
    if (!selected) return;

    const problem = preCheck(selected);
    setErrorMessage(problem);
    if (problem) {
      clearSelection();
      return;
    }

    setFile(selected);
    setPreviewUrl(URL.createObjectURL(selected));
  }

  function handleUpload() {
    if (!file) return;
    upload.mutate(file, {
      onSuccess: () => {
        clearSelection();
        showToast("Profile picture updated.", { variant: "success" });
      },
      onError: (error) => setErrorMessage(serverMessage(error)),
    });
  }

  const rawUrl = previewUrl ?? (avatarUrl === brokenUrl ? null : avatarUrl);
  const displayedUrl = rawUrl && isDisplayableMediaUrl(rawUrl) ? rawUrl : null;

  return (
    <section className="flex flex-col gap-4 border-b border-border pb-6">
      <div className="flex items-center gap-4">
        <div className="size-20 shrink-0 overflow-hidden rounded-full border border-border bg-surface">
          {displayedUrl ? (
            <img
              src={displayedUrl}
              alt="Profile picture"
              className="size-full object-cover"
              onError={() => {
                // A failed local blob preview must not mark the server's
                // avatar as broken — only the real avatarUrl can be broken.
                if (!previewUrl) setBrokenUrl(avatarUrl);
              }}
            />
          ) : (
            <div className="flex size-full items-center justify-center" aria-hidden="true">
              <UserRound className="size-8 text-text-muted" />
            </div>
          )}
        </div>
        <div>
          <h2 className="text-base font-semibold text-text">Profile picture</h2>
          <p className="text-sm text-text-muted">JPEG, PNG, or WebP — 5 MB maximum.</p>
        </div>
      </div>

      {file ? (
        <div className="flex gap-2">
          <Button onClick={handleUpload} isLoading={upload.isPending}>
            Upload photo
          </Button>
          <Button variant="secondary" onClick={clearSelection} disabled={upload.isPending}>
            Cancel
          </Button>
        </div>
      ) : (
        <FileDropzone
          accept={ACCEPTED_AVATAR_TYPES.join(",")}
          onFilesSelected={handleFilesSelected}
          hasError={Boolean(errorMessage)}
          label="Drag an image here, or click to choose one"
        />
      )}

      {errorMessage && <Alert variant="error">{errorMessage}</Alert>}
    </section>
  );
}

export { AvatarUpload };
