import { useState } from "react";
import { formatInitials } from "../../../shared/utils/format";
import { isRemoteMediaUrl } from "../../../shared/utils/mediaUrl";

interface OwnerAvatarProps {
  avatarUrl?: string | null;
  firstName?: string | null;
  lastName?: string | null;
}

// Hidden from screen readers: the owner's name is shown next to it.
function OwnerAvatar({ avatarUrl, firstName, lastName }: OwnerAvatarProps) {
  // Holds the URL that failed, so a different avatar URL gets loaded again.
  const [failedUrl, setFailedUrl] = useState<string | null>(null);
  const photoUrl =
    avatarUrl && avatarUrl !== failedUrl && isRemoteMediaUrl(avatarUrl) ? avatarUrl : null;

  return (
    <span className="flex size-11 flex-none items-center justify-center overflow-hidden bg-ink-900 text-sm font-semibold text-white">
      {photoUrl ? (
        <img
          src={photoUrl}
          alt=""
          className="size-full object-cover"
          onError={() => setFailedUrl(photoUrl)}
        />
      ) : (
        <span aria-hidden="true">{formatInitials(firstName, lastName) || "?"}</span>
      )}
    </span>
  );
}

export { OwnerAvatar };
