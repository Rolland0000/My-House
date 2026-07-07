---
trigger: always_on
---

## Project

MyHouse — real estate rental matching platform (owners ↔ seekers).
Target markets: France + francophone Africa. Two-person team, web-first (mobile is V2).

## Stack (locked)

- Backend: Rust / Axum / Tokio — modular monolith
- Frontend: React / TypeScript
- Database: PostgreSQL
- Storage: LocalFsStorage (MVP) → MinIO/S3 (V2), behind `StorageProvider` trait
- Infra: Docker / docker-compose

## Reference Documents

Authoritative specs — defer to these over general best practices when they conflict:

- `docs/ARCHITECTURE.md` (arc42, v2.1) — modules, ADRs, deployment
- `docs/TECHNICAL_SPEC_MVP.md` (v1.1) — DDL, API contract, folder structure
- `docs/MyHouse_CahierDesCharges_v2_0.docx` — functional requirements (ISO 29148)

## Key Decisions Already Locked — Do Not Re-litigate

- OTP passwordless auth, single endpoint (`is_new_user` bool)
- Roles: `seeker` (default) → `owner` (admin-validated request) → `admin`
- Owner upgrade: single atomic multipart submission, no draft/two-step
- Refresh tokens: rotating, 30-day sliding TTL, httpOnly/Secure/SameSite=Strict cookie
- `is_active` re-verified on every authenticated request (`AuthUser` extractor)
- Backend never proxies public file reads (listings/avatars served directly by nginx)
- Integration tests deferred to a dedicated end-of-roadmap epic — do not suggest adding them earlier

## Current Phase

EP-02 (Users module) — active implementation. Ticket decomposition for EP-00/EP-01 complete
and validated. All phases are now in scope: architecture, code, tests, deployment.

## Project-Specific Conventions

- Tickets: `MH-XXX` (two-digit epic padding from EP-01+), vertical slicing
  (`-BE`/`-FE` sub-tickets under one parent, never split by technical layer)
- GitHub tickets in English; conversation in French
- Library selection criteria (stability, maintainability, security) inform choices
  but aren't written into ticket descriptions — only crate name + purpose
- Validate structure in chat before generating any document artifact

## Available skills/rules and MCP (identify and invoke as needed for the specific task — do not load everything by default)
- general-code-guideline.md
- docker-rules.md
- readme-file-rules.md
- agent-react-typescrypt-rules.md
- database-rules.md
- rust-general-rules.md
- typecrypt-react-prompt-rules.md
- agent-react-typescrypt-rules.md
- ticket-creation-rules.md
By the end of the first prompt response of a new session, list all skills/rules and/or MCP include in the session

## Implementation depending on environment conditions (dev, staging, prod)
 Always make sure to respect implementaion depending of environment scope which is selected by the APP_ENV variable in .env. Code that will be always for local development phase, will always be selected when APP_ENV=development.
## Open Blockers

- Monetization model undefined — flag if a decision depends on it