
---
trigger: manual
---
---

description: General Rust rules for safe, idiomatic application and library development
globs: ["**/*.rs", "Cargo.toml", "Cargo.lock"]
----------------------------------------------

# Rust General Rules

## Project Structure

- Keep crates focused and name modules by domain responsibility.
- Put reusable library code in `src/lib.rs` and binary entry points in `src/main.rs` or `src/bin/`.
- Keep public APIs small and documented.
- Use feature flags deliberately and document non-default features.
- Commit `Cargo.lock` for applications; follow the project convention for libraries.

## Ownership and Types

- Prefer borrowing over cloning when ownership is not needed.
- Use owned values at API boundaries when the callee must store data.
- Model domain states with enums and structs instead of strings or booleans.
- Use `Option<T>` for absence and `Result<T, E>` for fallible operations.
- Avoid `unwrap()` and `expect()` outside tests, examples, and process-startup invariants.

## Error Handling

- Use `thiserror` or project-standard custom errors for libraries.
- Use `anyhow` or project-standard context-rich errors for applications.
- Add context when crossing IO, network, database, or parsing boundaries.
- Do not discard errors with `_` unless explicitly documented.

## Concurrency and Async

- Use `Send` and `Sync` boundaries intentionally.
- Prefer message passing or owned task inputs for async work.
- Do not hold blocking locks across `.await`.
- Use `tokio::task::spawn_blocking` or equivalent for blocking CPU or IO in async applications.
- Propagate cancellation through futures rather than hiding it in detached tasks.

## Testing and Quality

- Run `cargo check`, `cargo fmt` and `cargo clippy` before delivery (not cargo test or any other command that could take anough time).
- Add unit tests for pure logic and integration tests (*Integration tests deferred to a dedicated end-of-roadmap epic*) for public behavior.
- Use property tests for parsers, serializers, and state machines when useful.
- Use benchmarks only after identifying a real performance question.

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

## Common Mistakes

- Do not fight the borrow checker by adding unnecessary `Arc<Mutex<_>>`.
- Do not expose internal module structure through public APIs by accident.
- Do not allocate in hot loops without measuring.
- Do not use unsafe code unless the invariant is documented and tested.
