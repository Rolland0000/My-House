import { describe, expect, it } from "vitest";
import { withoutRow } from "./paginatedRows";

function page() {
  return {
    data: [{ id: "a" }, { id: "b" }],
    pagination: { page: 1, per_page: 20, total: 2, total_pages: 1 },
  };
}

describe("withoutRow", () => {
  it("removes the matching row and keeps the others", () => {
    const input = page();
    const output = withoutRow(input, "a");

    expect(output?.data).toEqual([input.data[1]]);
    expect(output?.data[0]).toBe(input.data[1]);
    expect(input.data).toHaveLength(2);
    expect(output?.pagination).toMatchObject({ total: 1, total_pages: 1 });
    expect(input.pagination.total).toBe(2);
  });

  it("drops the counts to zero when the last row goes", () => {
    const output = withoutRow(withoutRow(page(), "a"), "b");

    expect(output?.data).toEqual([]);
    expect(output?.pagination).toMatchObject({ total: 0, total_pages: 0 });
  });

  it("returns the input unchanged for an unknown id", () => {
    const input = page();
    expect(withoutRow(input, "missing")).toBe(input);
  });

  it("passes undefined through", () => {
    expect(withoutRow(undefined, "a")).toBeUndefined();
  });
});
