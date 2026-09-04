import { useEffect, useState } from "react";
import { UserRound } from "lucide-react";
import { Alert, Button, FileDropzone, useToast } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { ACCEPTED_AVATAR_TYPES, MAX_AVATAR_SIZE_BYTES } from "../../../shared/api/constants";
import { useUploadAvatar } from "../hooks/useUploadAvatar";

const FORMAT_MESSAGE = "Format non supporté. Choisissez une image JPEG, PNG ou WebP.";
const SIZE_MESSAGE = "Image trop lourde. La taille maximale est de 5 Mo.";

/** Pre-checks that spare an obviously doomed round-trip. The server re-runs
 *  both on the actual bytes, so a file that slips through here still gets
 *  rejected — see `serverMessage`. */
function preCheck(file: File): string | null {
  if (!ACCEPTED_AVATAR_TYPES.includes(file.type)) return FORMAT_MESSAGE;
  if (file.size > MAX_AVATAR_SIZE_BYTES) return SIZE_MESSAGE;
  return null;
}

/** Distinct messages per backend error code (`shared/errors.rs`); a size
 *  rejection must never read as a format problem. */
function serverMessage(error: unknown): string {
  if (!(error instanceof ApiError)) return "Envoi impossible. Réessayez.";
  if (error.code === "PAYLOAD_TOO_LARGE") return SIZE_MESSAGE;
  if (error.code === "INVALID_FILE") return FORMAT_MESSAGE;
  if (error.code === "BAD_REQUEST") return "Fichier illisible. Sélectionnez une autre image.";
  return error.message;
}

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
        showToast("Photo de profil mise à jour.", { variant: "success" });
      },
      onError: (error) => setErrorMessage(serverMessage(error)),
    });
  }

  const displayedUrl = previewUrl ?? (avatarUrl === brokenUrl ? null : avatarUrl);

  return (
    <section className="flex flex-col gap-4 border-b border-border pb-6">
      <div className="flex items-center gap-4">
        <div className="size-20 shrink-0 overflow-hidden rounded-full border border-border bg-surface">
          {displayedUrl ? (
            <img
              src={displayedUrl}
              alt="Photo de profil"
              className="size-full object-cover"
              onError={() => setBrokenUrl(avatarUrl)}
            />
          ) : (
            <div className="flex size-full items-center justify-center" aria-hidden="true">
              <UserRound className="size-8 text-text-muted" />
            </div>
          )}
        </div>
        <div>
          <h2 className="text-base font-semibold text-text">Photo de profil</h2>
          <p className="text-sm text-text-muted">JPEG, PNG ou WebP — 5 Mo maximum.</p>
        </div>
      </div>

      {file ? (
        <div className="flex gap-2">
          <Button onClick={handleUpload} isLoading={upload.isPending}>
            Envoyer la photo
          </Button>
          <Button variant="secondary" onClick={clearSelection} disabled={upload.isPending}>
            Annuler
          </Button>
        </div>
      ) : (
        <FileDropzone
          accept={ACCEPTED_AVATAR_TYPES.join(",")}
          onFilesSelected={handleFilesSelected}
          hasError={Boolean(errorMessage)}
          label="Glissez une image ici, ou cliquez pour en choisir une"
        />
      )}

      {errorMessage && <Alert variant="error">{errorMessage}</Alert>}
    </section>
  );
}

export { AvatarUpload };
