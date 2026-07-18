
---
name: code-review-backend
description: Use when reviewing a MyHouse backend PR or diff (Rust/Axum/sqlx) before merge — checks correctness, security, idiomatic Rust, and conformance to ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md. Trigger on "review this PR", "review my backend changes", a pasted diff/branch touching backend/src/**, or a MH-XX-BE ticket marked ready for review. Do NOT use for reviewing frontend code (use code-review-frontend) or for writing new features.
---
# Code Review — Backend (Rust / Axum / MyHouse)

## Purpose

Produce a structured, consistent review of backend changes: catch bugs, security issues,
and idiomatic-Rust violations a tired human reviewer might miss — **and** verify the diff
doesn't quietly drift from the locked architecture (module boundaries, layering, storage
abstraction, auth invariants).

Full checklists (Rust idioms, security patterns, MyHouse architecture rules) live in
`reference/checklist.md` — load it before starting Phase 3. Don't paraphrase this file from
memory; read it fresh each time since it may be updated independently of this skill.

## When NOT to trigger

- Reviewing frontend/TS code → `code-review-frontend`
- Writing/implementing a ticket → normal coding flow, not this skill
- Reviewing infra/Docker-only changes with no Rust code → light pass is fine, skip Phase 3's Rust-specific checks

## Process — Four Phases

Don't skip phases. Don't jump straight to line comments — Phase 1–2 catch scope and
architecture problems that make line-level nitpicking moot.

### Phase 1 — Context

- What ticket (MH-XX) does this PR close? Read the ticket description if available.
- What module(s) does it touch? (`auth`, `users`, `listings`, `search`, `media`, `contact`,
  `notifications`, `admin`, `infra`, `shared`)
- Is the change a vertical slice matching one ticket, or does it leak into unrelated
  modules/scope? Flag scope creep as `important` — it's a MyHouse convention violation,
  not just a style nit.
- Read the diff in full once before commenting on anything.

### Phase 2 — High-Level Review

- **Layering**: does `handler.rs` stay thin (HTTP concerns only, delegates to `service.rs`)?
  Does `service.rs` avoid importing `axum::*`? Does `repository.rs` contain the SQL, not
  `service.rs`?
- **Module boundaries**: does this module import another module's concrete type (e.g.
  `media::repository::MediaRepo`) instead of going through a trait/service interface in
  `shared/`? That's a boundary violation per ARCHITECTURE.md §4.2/§5.3.
- **Storage abstraction**: any new code touching files must go through
  `Arc<dyn StorageProvider>`, never `std::fs` directly outside `infra/storage/local_fs.rs`.
- **Migration hygiene**: new `sqlx` migration is timestamped, additive-only unless explicitly
  a breaking change discussed with the team.

### Phase 3 — Line-by-Line

Load `reference/checklist.md` now. Go through: logic correctness, error handling
(`AppError`, no naked `unwrap()`/`expect()`/`panic!()` in request-handling paths without a justified reason), security
(SQL injection via `sqlx` query building, auth bypass, path traversal in storage keys),
`unsafe` blocks (mandatory `// SAFETY:` comment justifying every one), async correctness
(no blocking calls in async fn, no holding a lock across `.await`), naming and idiom fit.

### Phase 4 — Summary & Decision

Output the report (format below) directly in chat as markdown — never auto-post to GitHub
even if `gh` CLI is available, unless the person explicitly asks you to post it.

## Severity Labels

| Label          | Meaning                                                                                            |
| -------------- | -------------------------------------------------------------------------------------------------- |
| `blocking`   | Must fix before merge — bug, security hole, architecture violation, broken invariant              |
| `important`  | Should fix — scope creep, missing test, weak error handling, non-idiomatic pattern with real cost |
| `nit`        | Style/naming preference, no functional impact                                                      |
| `suggestion` | Optional improvement, not required for this PR                                                     |

## Report Format

```markdown
## Review — MH-XX (module)

**Scope check:** [matches ticket / scope creep into <module>]
**Architecture check:** [clean / N violations — see below]

### Findings

**[blocking] path/to/file.rs:42**
<what's wrong, why it matters, suggested fix>

**[important] path/to/file.rs:88**
...

### Summary
X blocking · Y important · Z nit/suggestion
Verdict: [approve / approve with comments / request changes]
```

## MyHouse-Specific Invariants (always check, even in a small diff)

- `is_active` re-verified by `AuthUser` extractor on every authenticated route — never
  bypassed by a handler pulling the user from a raw JWT claim decode.
- Refresh token rotation: any `/auth/refresh` logic must revoke the consumed token
  (`revoked_at`) and chain `replaced_by_id` — reuse of a revoked token must revoke the whole
  family, not just reject the single request.
- Public files (`listings/*`, `avatars/*`) are never re-served through a backend endpoint —
  only `owner-requests/*` documents go through an authenticated proxy read, and only for
  `admin` role.
- Storage keys are always server-generated (UUID), never derived from client-supplied
  filenames — path traversal check.
