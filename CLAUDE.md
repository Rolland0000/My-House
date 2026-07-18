# Project

MyHouse — real estate rental matching platform (owners ↔ seekers).
Target markets: France + francophone Africa. Two-person team, web-first (mobile is V2).

## Stack (locked)

- Backend: Rust / Axum / Tokio — modular monolith
- Frontend: React / TypeScript
- Database: PostgreSQL
- Storage: LocalFsStorage (MVP) → MinIO/S3 (V2), behind `StorageProvider` trait
- Cache: moka (in-memory) — no Redis at MVP
- Infra: Docker / docker-compose

## Reference Documents

Authoritative specs — defer to these over general best practices when they conflict:

- `docs/ARCHITECTURE.md` (arc42, v2.1) — modules, ADRs, ADRs, runtime flows, deployment. Read before any cross-module or architectural change.
- `docs/TECHNICAL_SPEC_MVP.md` (v1.1) — DDL, API contract, folder structure. Read before touching schema, endpoints, or module layout.
- `docs/MyHouse_CahierDesCharges_v2_0.docx` — functional requirements (ISO 29148). Source of truth for feature scope.

## Key Decisions Already Locked — Do Not Re-litigate

- OTP passwordless auth, single endpoint (`is_new_user` bool)
- Roles: `seeker` (default) → `owner` (admin-validated request) → `admin`
- Owner upgrade: single atomic multipart submission, no draft/two-step
- Refresh tokens: rotating, 30-day sliding TTL, httpOnly/Secure/SameSite=Strict cookie
- File type validation via magic bytes (`infer` crate), never by extension or declared Content-Type
- Uploaded filenames are ALWAYS server-generated (UUID), never derived from client input (path-traversal prevention)
- `is_active` re-verified on every authenticated request (`AuthUser` extractor)
- Backend never proxies public file reads (listings/avatars served directly by nginx)
- Integration tests deferred to a dedicated end-of-roadmap epic — do not suggest adding them earlier

## Project-Specific Conventions

- Tickets: `MH-XXX` (two-digit epic padding from EP-01+), vertical slicing
  (`-BE`/`-FE` sub-tickets under one parent, never split by technical layer)
- GitHub tickets in English; conversation in French
- Library selection criteria (stability, maintainability, security) inform choices
  but aren't written into ticket descriptions — only crate name + purpose
- Validate structure in chat before generating any document artifact

## Available skills/rules and MCP (identify and invoke as needed for the specific task — do not load everything by default)

- general-code-guideline.md
- agent-react-typescrypt-rules.md
- database-rules.md
- rust-general-rules.md

By the end of the first prompt response of a new session, list all skills/rules and/or MCP include in the session

## Architecture invariants

* Modular monolith: modules interact only via service traits (in `shared/`) or bootstrap injection. A module never imports another module's concrete impl.
* Layering per module: `handler` (HTTP, no business logic) → `service` (business logic, no Axum) → `repository` (data access, trait). Respect these boundaries.
* Errors converge to a central `AppError` → structured HTTP response.

## Implementation depending on environment conditions (dev, staging, prod)

 Always make sure to respect implementaion depending of environment scope which is selected by the APP_ENV variable in .env. Code that will be always for local development phase, will always be selected when APP_ENV=development.

## MCP usage policy

- **GitHub MCP** — use for ticket creation, PR review, issue lookup. Never push or merge
  without explicit user confirmation for that specific action.
- **PostgreSQL MCP** — consult before writing/modifying a query touching `listings` or
  `search` to check existing indexes and schema state (see .claude/rules/database.md).
  Read-only exploration by default; never run destructive statements (DROP/TRUNCATE/DELETE
  without WHERE) without explicit confirmation.
- **Git MCP** — status/diff/log/branch only. NEVER commit or push automatically, even if a
  task seems complete — always stop and let the user commit.
- **Context7** — consult when working with Axum, sqlx, or React APIs where version-specific
  behavior matters (e.g. sqlx macro syntax, Axum extractor signatures) — don't rely on
  training-data memory for library APIs that change across versions.
- **Filesystem MCP vs. native file tools** — prefer native tools for MyHouse repo files
  unless a task specifically requires the Filesystem MCP's capabilities.

- **Sequential Thinking MCP** — use only for genuinely multi-step architectural decisions
  (e.g. planning the atomic owner-request flow, refresh token rotation edge cases). Cap at
  ~8-10 thoughts; if a chain isn't converging, stop and ask the user instead of continuing.
  Never use it for simple/single-step tasks (routine CRUD, small bugfixes).

## Open Blockers

- Monetization model undefined — flag if a decision depends on it
