
# Frontend Review Checklist — Detail Reference

Load this only during Phase 3 of the review process. Curated subset relevant to a
React/TypeScript/Vite SPA — not a full framework-agnostic checklist (see
Front-End-Checklist project for that scope if a broader audit is ever needed).

---

## 1. React Correctness & Hooks

- [ ] Hooks called unconditionally at the top level — no hook inside a condition, loop, or
  after an early `return`.
- [ ] Dependency arrays (`useEffect`, `useMemo`, `useCallback`) are complete and honest — no
  suppressed `eslint-disable-next-line react-hooks/exhaustive-deps` without a comment
  explaining why it's safe.
- [ ] Effects that fetch data handle cleanup/cancellation (abort on unmount or param change)
  to avoid setting state on an unmounted component.
- [ ] Keys in lists are stable and unique (entity `id`, not array index) unless the list is
  provably static and never reordered.
- [ ] Controlled vs uncontrolled inputs not mixed within the same form field.
- [ ] No derived state duplicated in `useState` when it can be computed inline or via
  `useMemo` from existing props/state — avoids sync bugs.

## 2. TypeScript Strictness

- [ ] No `any` introduced without a comment justifying it (third-party type gap, etc.) —
  prefer `unknown` + narrowing, or the actual generated type from `types.ts`.
- [ ] API response/request shapes use the generated types from `shared/api/types.ts`, not a
  hand-rolled interface that could drift from the OpenAPI contract.
- [ ] No unchecked `as` cast on data crossing a trust boundary (API response, `FormData`
  values) — validate or narrow instead of asserting.
- [ ] Discriminated unions used for state that has distinct shapes per case (e.g. owner
  request status `pending | approved | rejected`) rather than optional-everything.

## 3. State & Data Fetching

- [ ] Server state (listings, search results, profile) goes through the data-fetching hook
  pattern already established in the feature (`hooks/useListings.ts`, etc.), not a raw
  `fetch` scattered in a component.
- [ ] Loading, error, and empty states are all handled explicitly for anything that fetches —
  not just the happy path. Missing error state is `important`.
- [ ] Optimistic updates (if any) have a rollback path on failure.
- [ ] Pagination state matches the backend contract shape (`page`, `per_page`, `total`,
  `total_pages`) — no reinventing pagination math client-side that duplicates
  `pagination.ts`.

## 4. Accessibility (baseline, not full WCAG audit)

- [ ] Interactive elements are real `<button>`/`<a>`, not `<div onClick>` — if a `<div>` must
  be clickable, it has `role`, `tabIndex`, and keyboard handlers.
- [ ] Images have meaningful `alt` text (cover photos: listing title; avatars: user name;
  decorative icons: `alt=""`).
- [ ] Form inputs have associated `<label>` (via `htmlFor`/`id` or wrapping), not placeholder
  text standing in for a label.
- [ ] Color is not the only signal for state (e.g. listing status `available`/`unavailable`
  needs a text/icon cue too, not just a color chip).
- [ ] Focus is managed sensibly after modal open/close and route transitions — no focus lost
  to `<body>`.

## 5. Performance

- [ ] No obviously expensive computation (sort/filter/map over a large list) re-run every
  render without memoization when the inputs are stable — flag as `important` only if
  the list is unbounded/user-scale (feed, search results), `nit` otherwise; premature
  `useMemo` everywhere is its own anti-pattern.
- [ ] Images (listing photos) have explicit `width`/`height` or aspect-ratio box to avoid
  layout shift, and lazy-load below the fold (`loading="lazy"`) on feed/grid views.
- [ ] No unnecessary prop-drilling causing broad re-render cascades where context or
  colocation would be simpler — but don't suggest introducing new global state for a
  two-level prop pass.

## 6. Security (Frontend-Relevant)

- [ ] No `dangerouslySetInnerHTML` with unsanitized user/API content (listing descriptions,
  owner notes) — if rendering rich text is genuinely needed, it must go through a
  sanitizer, and that's a deliberate, discussed decision, not a quick fix.
- [ ] No token, secret, or PII logged to `console.*` left in the diff.
- [ ] External links (if any, e.g. future map/profile links) use
  `rel="noopener noreferrer"` with `target="_blank"`.

## 7. MyHouse Project Structure

Map for reference:

```
app/            — router, providers, layout shells (RootLayout, AuthLayout, AdminLayout)
features/auth/       — OTP request/verify, profile setup
features/listings/   — feed, card, detail, owner CRUD form
features/search/     — search bar, filter panel
features/profile/    — profile form, owner-request form, avatar upload
features/contact/    — contact reveal
features/admin/      — user/listing/owner-request tables
shared/components/   — generic UI primitives (Button, Card, Modal, Pagination, Spinner)
shared/api/          — client.ts (fetch instance) + types.ts (generated, do not edit)
```

- [ ] New component lives in the feature folder matching its domain — a `ListingCard` doesn't
  belong in `shared/components/` unless it's genuinely reused across ≥2 unrelated
  features.
- [ ] `shared/api/types.ts` never hand-edited — any type mismatch traces back to a backend
  `utoipa` annotation gap, not a local patch.
- [ ] `shared/api/client.ts` is the single fetch entry point — no feature reimplementing base
  URL, headers, or auth-refresh interception locally.
- [ ] Role-gated UI (owner-only forms, `/admin/*`) has both: (a) a route guard in
  `app/router.tsx`, and (b) conditional rendering for UX — one without the other is a
  `blocking` finding (either a security gap or a confusing dead end).
- [ ] Public media (`listing photo`, `avatar_url`) rendered via plain `<img src={url}>`
  pointing at the stored URL — never wrapped in a component that re-fetches through the
  backend API.
- [ ] `AvatarUpload.tsx` / `ListingForm.tsx` photo inputs respect the backend constraints
  surfaced in UI (max 5 photos, 5 MB, JPEG/PNG/WebP) with client-side validation as a UX
  nicety — but the review should confirm the UI doesn't claim to be the source of truth
  for validation (server still validates via magic bytes).

## 8. UX Copy (light pass)

- [ ] Error messages shown to users are the `message` from the API error contract, or a
  sensible French fallback — not a raw exception string or English leaking into a
  French-language UI.
- [ ] Empty states (empty feed, no search results, no listings yet for an owner) have
  copy, not just a blank area.
