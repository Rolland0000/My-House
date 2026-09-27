export type ListingVisibility = "draft" | "published" | "hidden";

interface ListingVisibilityInput {
  publishedAt: string | null;
  hasPhoto: boolean;
}

export function listingVisibility({
  publishedAt,
  hasPhoto,
}: ListingVisibilityInput): ListingVisibility {
  if (publishedAt === null) return "draft";
  return hasPhoto ? "published" : "hidden";
}
