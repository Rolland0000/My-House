import { describe, expect, it } from "vitest";
import type { ListingMedia } from "./api";
import { movePhoto, withCoverFirst } from "./photoOrder";

function photo(id: string, position: number, is_cover = false): ListingMedia {
  return { id, position, is_cover, url: `https://cdn.example/${id}.jpg` };
}

const media = [photo("cover", 0, true), photo("a", 1), photo("b", 2), photo("c", 3)];

const ids = (list: ListingMedia[] | null) => list?.map(({ id }) => id);

describe("movePhoto", () => {
  it("moves a photo earlier", () => {
    expect(ids(movePhoto(media, "b", -1))).toEqual(["cover", "b", "a", "c"]);
  });

  it("moves a photo later", () => {
    expect(ids(movePhoto(media, "a", 1))).toEqual(["cover", "b", "a", "c"]);
  });

  it("renumbers positions from the new order", () => {
    const moved = movePhoto([photo("cover", 0, true), photo("a", 4), photo("b", 4)], "b", -1);
    expect(moved?.map(({ id, position }) => [id, position])).toEqual([
      ["cover", 0],
      ["b", 1],
      ["a", 2],
    ]);
  });

  it("refuses to move past the end", () => {
    expect(movePhoto(media, "c", 1)).toBeNull();
  });

  it("refuses to move onto the cover's place", () => {
    expect(movePhoto(media, "a", -1)).toBeNull();
  });

  it("never moves the cover", () => {
    expect(movePhoto(media, "cover", 1)).toBeNull();
  });

  it("refuses to move past the start when there is no cover", () => {
    expect(movePhoto([photo("a", 0), photo("b", 1)], "a", -1)).toBeNull();
  });

  it("ignores an unknown id", () => {
    expect(movePhoto(media, "missing", 1)).toBeNull();
  });
});

describe("withCoverFirst", () => {
  it("leaves a list that already starts with the cover unchanged", () => {
    expect(ids(withCoverFirst(media))).toEqual(["cover", "a", "b", "c"]);
  });

  it("moves a cover from the middle to the front", () => {
    const list = [photo("a", 0), photo("b", 1, true), photo("c", 2)];
    expect(ids(withCoverFirst(list))).toEqual(["b", "a", "c"]);
  });

  it("keeps the order when there is no cover", () => {
    const list = [photo("a", 0), photo("b", 1)];
    expect(ids(withCoverFirst(list))).toEqual(["a", "b"]);
  });
});
