import type { BadgeTone } from "../../shared/components";
import type { AdminListingVisibility } from "./api";

export const ADMIN_VISIBILITY_BADGE: Record<
  AdminListingVisibility,
  { tone: BadgeTone; label: string }
> = {
  draft: { tone: "warning", label: "Draft" },
  no_photo: { tone: "error", label: "No photo" },
  public: { tone: "success", label: "Public" },
  hidden: { tone: "neutral", label: "Hidden" },
};
