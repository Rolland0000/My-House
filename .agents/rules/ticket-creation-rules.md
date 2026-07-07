---
trigger: manual
---

# Skill: GitHub Ticket Generation (MyHouse)

## When to use
Decomposing an epic into tickets, or drafting a single ticket from a feature/spec.

## Ticket format

```
**MH-XX: [Imperative title]**
Epic: EP-XX
Workstream: BE | FE | BOTH
Estimate: X day(s)
Stack: [crate/lib names only — no selection rationale]
Dependencies: MH-XX (if any)

**Context**
1-3 sentences: why this ticket exists, what it enables. Clear and understandable description of the task the ticket handle.

**Acceptance Criteria**
- [ ] Concrete, testable outcome
- [ ] ...

**Technical Notes**
Implementation detail only if non-obvious from ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md.
Omit this section if nothing beyond the spec needs saying.
```

## Rules

- **Vertical slicing**: if a ticket needs both BE and FE work, split into `MH-XX-BE` and 
  `MH-XX-FE` under the same parent number. Never split a ticket by technical layer alone 
  (e.g. never "handler.rs ticket" + "service.rs ticket").
- **One atomic outcome per ticket.** If acceptance criteria need "and" to connect two unrelated 
  behaviors, split the ticket.
- **Two-digit epic padding** from EP-01 onward (MH-01 → MH-99 per epic namespace, not global).
- **English only** — title, context, AC, notes.
- **No library justification** in the ticket — crate name + one-line purpose only 
  (e.g. `sqlx — Postgres async driver`, not why it was chosen over alternatives).
- Reference `ARCHITECTURE.md` / `TECHNICAL_SPEC_MVP.md` for schema, endpoints, and module 
  boundaries — don't restate their content, cite the section if needed 
  (e.g. "see §4.2bis Owner Requests").
- Do not add integration tests as acceptance criteria — deferred to the dedicated test epic 
  (per project instructions).

## Output when decomposing an epic

1. List proposed tickets as a flat numbered list (title + workstream only) first.
2. Wait for validation before writing full ticket bodies.
3. Once validated, output full tickets in the format above, one per ticket.

## Estimate calibration

- 0.5 day: single CRUD endpoint, single component, single migration
- 1 day: endpoint + validation logic, form with multiple fields, repository + tests scaffold
- 2 days: multi-step flow (e.g. atomic multipart submission), module with several endpoints
- 3+ days: flag as candidate for further splitting — justify why it can't be broken down