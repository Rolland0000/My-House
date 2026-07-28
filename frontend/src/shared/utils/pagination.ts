export type PageItem = number | "ellipsis-start" | "ellipsis-end";

function range(start: number, end: number): number[] {
  return Array.from({ length: end - start + 1 }, (_, index) => start + index);
}

export function getPageItems(page: number, totalPages: number, siblingCount: number): PageItem[] {
  const totalItemsWithoutEllipsis = siblingCount * 2 + 5;

  if (totalItemsWithoutEllipsis >= totalPages) {
    return range(1, totalPages);
  }

  const leftSiblingIndex = Math.max(page - siblingCount, 1);
  const rightSiblingIndex = Math.min(page + siblingCount, totalPages);

  const showLeftEllipsis = leftSiblingIndex > 2;
  const showRightEllipsis = rightSiblingIndex < totalPages - 1;

  if (!showLeftEllipsis && showRightEllipsis) {
    const leftRange = range(1, 3 + 2 * siblingCount);
    return [...leftRange, "ellipsis-end", totalPages];
  }

  if (showLeftEllipsis && !showRightEllipsis) {
    const rightRange = range(totalPages - (3 + 2 * siblingCount) + 1, totalPages);
    return [1, "ellipsis-start", ...rightRange];
  }

  return [
    1,
    "ellipsis-start",
    ...range(leftSiblingIndex, rightSiblingIndex),
    "ellipsis-end",
    totalPages,
  ];
}
