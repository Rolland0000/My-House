
**Scope:** `backend/migrations/**`, `backend/src/**/repository.rs`, `**/*.sql`

# Database Rules — sqlx / PostgreSQL

MyHouse uses `sqlx` directly against PostgreSQL — no ORM (Prisma/Supabase were evaluated and
rejected, see ARCHITECTURE.md ADR context / project memory). These rules replace generic
ORM-oriented advice.

## Migrations

- One timestamped file per migration under `backend/migrations/` (`sqlx migrate add <name>`).
- Migrations are forward-only in this repo — no down-migrations authored for MVP; revert via a
  new forward migration.
- Never edit a migration that has been applied in any shared environment (dev DB, CI). Add a new one.

## Queries

- Prefer `sqlx::query!` / `query_as!` (compile-time checked against the DB schema) over
  untyped `query()` — catches column/type drift at `cargo build` time.
- Requires `DATABASE_URL` reachable at compile time (see `sqlx-cli` / `cargo sqlx prepare` for
  CI without a live DB — check if `.sqlx/` offline cache is in use before assuming a live DB).
- One repository method = one query intent. Don't build ad-hoc dynamic SQL string concatenation
  for filters — use `QueryBuilder` if a query needs optional/combinable filters (e.g. search
  filters in `TECHNICAL_SPEC_MVP.md §4.4`).

## Schema conventions (already locked — see TECHNICAL_SPEC_MVP.md §2)

- `UUID PRIMARY KEY DEFAULT uuid_generate_v4()` on every table.
- `created_at` / `updated_at TIMESTAMPTZ` — `updated_at` maintained by trigger
  (`fn_set_updated_at`), never set manually in application code.
- Enums as PostgreSQL `ENUM` types, not `VARCHAR` + check constraint.
- Partial unique indexes for "at most one active X" invariants (e.g. one pending
  `owner_request` per user, one cover photo per listing) — enforce in DB, not just in service
  logic.

## Cascades and cleanup

- `ON DELETE CASCADE` handles relational cleanup only. It never touches the filesystem/object
  storage — physical file deletion (`StorageProvider::delete()`) must be called explicitly
  *before* the SQL `DELETE`, in the service layer. See ARCHITECTURE.md §8.1 "Suppression de
  compte".
- Never rely on cascade order to guarantee business invariants — make the sequence explicit in
  the service.

## Performance

- Every new query touching `listings` or `search` should be checked against existing indexes
  (`idx_listings_search`, `idx_listings_owner`, `idx_listings_city`, `idx_listings_status`,
  `idx_listings_type`) before adding a new one.
- No index on `listings.price` yet (known gap, see ARCHITECTURE.md R-07) — flag if a new query
  filters/sorts by price at scale, don't silently add the index as a side effect of unrelated work.
- Connection pooling via `sqlx::PgPool` — configured once in `infra/db.rs`. Don't create ad-hoc
  pools or raw connections elsewhere.

## Security

- Never string-interpolate user input into SQL. `sqlx::query!`/`query_as!` parameterize by
  construction — if you ever reach for `format!()` to build a query, stop and use `QueryBuilder`
  instead.
- Sensitive columns (`identity_data`, `identity_documents` JSONB in `owner_requests`) are never
  selected in list/search queries — only in the admin-scoped single-record read
  (`GET /admin/owner-requests/:id/documents/:doc_id`).

## Testing

- Repository tests run against a real test DB inside a transaction that's rolled back
  (`#[cfg(test)]`, feature flag `integration` — see TECHNICAL_SPEC_MVP.md §6.1). Never mock the
  DB layer in repository tests; mocking belongs at the service layer (repository trait mocked).
