import type { ListingMedia } from "./api";

export type MoveDirection = -1 | 1;

/** Returns null when the move isn't allowed: the cover, an unknown id, or a step past either end or onto the cover. */
export function movePhoto(
  media: ListingMedia[],
  mediaId: string,
  direction: MoveDirection
): ListingMedia[] | null {
  const from = media.findIndex(({ id }) => id === mediaId);
  const to = from + direction;
  if (from === -1 || media[from].is_cover || to < 0 || to >= media.length || media[to].is_cover) {
    return null;
  }
  const reordered = [...media];
  [reordered[from], reordered[to]] = [reordered[to], reordered[from]];
  return reordered.map((photo, index) => ({ ...photo, position: index }));
}

/** Moves the cover to the front and keeps the other photos in their order. */
export function withCoverFirst(media: ListingMedia[]): ListingMedia[] {
  const cover = media.find(({ is_cover }) => is_cover);
  return cover ? [cover, ...media.filter((photo) => photo !== cover)] : media;
}

/** Identifies a move button, so the page can focus it again after its tile moves. */
export function moveControl(mediaId: string, direction: MoveDirection): string {
  return `${mediaId}:${direction}`;
}
