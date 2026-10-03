import type { BadgeTone } from "../../shared/components";
import type { ListingStatus, ListingType } from "./api";
import type { ListingVisibility } from "./listingVisibility";

interface BadgeSpec {
  tone: BadgeTone;
  label: string;
}

export const typeLabels: Record<ListingType, string> = {
  apartment: "Apartment",
  studio: "Studio",
  house: "House",
  room: "Room",
  villa: "Villa",
  other: "Other",
};

export const VISIBILITY_BADGE: Record<ListingVisibility, BadgeSpec> = {
  draft: { tone: "warning", label: "Draft" },
  published: { tone: "success", label: "Published" },
  hidden: { tone: "error", label: "Hidden — no photos" },
};

export const AVAILABILITY_BADGE: Record<ListingStatus, BadgeSpec> = {
  available: { tone: "success", label: "Available" },
  unavailable: { tone: "error", label: "Unavailable" },
};
