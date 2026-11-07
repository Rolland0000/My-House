interface PaginatedRows<T> {
  data: T[];
  pagination: { page: number; per_page: number; total: number; total_pages: number };
}

/** Returns a copy without the matching row. The counts drop with it so that removing the
 *  last row shows the empty state right away. */
export function withoutRow<T extends { id: string }, R extends PaginatedRows<T>>(
  result: R | undefined,
  id: string
): R | undefined {
  if (!result) return result;
  const data = result.data.filter((row) => row.id !== id);
  if (data.length === result.data.length) return result;

  const total = Math.max(result.pagination.total - 1, 0);
  const totalPages = Math.ceil(total / result.pagination.per_page);
  return { ...result, data, pagination: { ...result.pagination, total, total_pages: totalPages } };
}
