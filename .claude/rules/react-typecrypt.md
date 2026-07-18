**Scope:** `frontend/src/**/*.tsx`, `frontend/src/**/*.ts`

# React / TypeScript Rules

Replaces `agent-react-typescrypt-rules.md` and `typecrypt-react-prompt-rules.md` — merged and
corrected against `TECHNICAL_SPEC_MVP.md §1.2` (authoritative folder structure) and current
best practices.

**Corrections made vs. the original two files:**

- Removed all "App Router" references — that's Next.js. MyHouse is a Vite SPA with
  `app/router.tsx` (React Router), not a Next.js app. See TECHNICAL_SPEC_MVP.md §1.2.
- Dropped `React.FC` as the recommended pattern — it's fallen out of favor (implicit
  `children`, awkward with generics). Use a plain typed function component instead.
- Replaced the generic `feature/{components,hooks,pages,types,utils}` folder sketch with the
  actual locked structure below — don't reinvent it per-feature.

## Folder structure — defer to TECHNICAL_SPEC_MVP.md §1.2, don't improvise

```
src/
  app/                  # router.tsx, providers.tsx, layout/
  features/<domain>/    # components/, hooks/use<X>.ts, api.ts, index.ts
  shared/
    components/
    hooks/
    api/                # client.ts, types.ts (generated — never hand-edit types.ts)
    utils/
```

One folder per domain under `features/` (`auth`, `listings`, `search`, `profile`, `contact`,
`admin`). A feature never imports another feature's internals directly — go through `shared/`
or the feature's public `index.ts`.

## Components

- Functional components only. Type props with a plain `interface`/`type`, not `React.FC<Props>`:
  ```tsx
  interface ListingCardProps { listing: Listing; onSelect?: (id: string) => void; }
  function ListingCard({ listing, onSelect }: ListingCardProps) { ... }
  ```
- `.tsx` extension for any file containing JSX.
- `React.memo` only when a measured re-render cost justifies it — not by default.
- Custom hooks (`useX`) for logic reused across components; keep one hook = one concern.

## TypeScript

- Strict mode on (`tsconfig.json` — don't relax it to silence errors).
- Prefer type inference where the type is obvious; annotate at function/API boundaries.
- Import shared types from `shared/api/types.ts` (generated from OpenAPI) — never redefine a
  type that already exists there.

## Performance & robustness

- `React.lazy` + `Suspense` for route-level code-splitting (per top-level route in
  `app/router.tsx`).
- Error boundaries around feature roots so one feature crashing doesn't blank the whole app.
- ESLint with the TypeScript + React plugin set — treat lint errors as build blockers, not
  warnings to defer.

## Data fetching

- Each feature's `api.ts` is the only place that calls `shared/api/client.ts`. Components never
  call `fetch`/the API client directly.
