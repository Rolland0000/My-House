# Handoff: My House — "Plan & Brass" design system + MVP screens

## Overview
Visual design and UI structure for My House, a rental listing platform connecting owners and seekers across France and French-speaking Africa. Covers the full MVP screen set from the design brief: role-based headers, 3-step auth, public feed + listing detail, owner property management, owner verification request, admin back-office, plus profile/account-deletion and a toast/modal inventory added in a follow-up round.

## About the design files
The `.dc.html` files in this bundle are **design references built in HTML** — high-fidelity visual/interaction mockups, not production code to copy verbatim. The task is to **recreate this design inside the existing My House frontend** (`Rolland0000/My-House`, branch `develop`, Tailwind utility-first, React), reusing the codebase's real shared components (`Button`, `Card`, `Modal`, `Alert`, `Input`, `Select`, `TextArea`, `FileDropzone`, `Pagination`, `FormField`) exactly as their current props/API are defined — this design does not rename or restructure them. New components proposed below (none exist in the repo yet) should be added under `frontend/src/shared/components/`.

## Fidelity
**High-fidelity.** Exact hex values, a defined type scale, and named spacing/radius/shadow tokens are given below. Recreate pixel-close using Tailwind classes mapped to these values (see Design Tokens) — don't reinterpret the palette or type scale.

## Design tokens

### Colour
| Token | Hex | Usage |
|---|---|---|
| `ink-900` | `#16233D` | Headings, dark section backgrounds (admin header) |
| `ink-600` | `#2E4A73` | Links, secondary/outline buttons, focus ring |
| `ink-500` | `#55606F` | Metadata text (5.4:1 on white) |
| `brass-500` | `#BE8A2E` | Single accent — primary CTA, verified seal, price plate border. Label text on brass is always `ink-900`, never white (white-on-brass is 2.4:1, under AA) |
| `brass-600` | `#9C7327` | Brass hover/active + border under brass fills |
| `paper-50` | `#F4F5F2` | Page background |
| `text-900` | `#1A1D1B` | Body text (15.8:1 on paper) |
| `rule-300` | `#D8DAD4` | Default 1px border (cards, fields, table rows) |
| `rule-400` | `#BDC1BA` | Hover border, dimension-rule line, dropzone border |
| success | `#2F6E4E` (text/icon) on `#EAF0EC` fill, border `rgba(47,110,78,.25–.3)` | Approved / available / published |
| warning | `#C77D1D` icon, `#8F5A12` text on `#F8F1E5` fill, border `rgba(199,125,29,.35)` | Pending / caution |
| error | `#B94A3D` icon, `#9A3A2E` text on `#F7EBE9` fill, border `rgba(185,74,61,.35)` | Rejected / destructive / failure |
| info | `#2E4A73` icon on `#EEF1F5` fill, border `rgba(46,74,115,.3)` | Neutral notices |

Cards are pure white `#FFFFFF` on `paper-50` — depth comes from that contrast, not shadow.

### Typography
Single family: **Archivo** (variable, weights 400/500/600/700/800). Load one variable font file only.
| Role | Size/weight | Line-height |
|---|---|---|
| display | 34px / 800 | 1.12, letter-spacing -.015em |
| h1 | 26px / 700 | 1.2 |
| h2 | 19px / 700 | 1.25 |
| body | 15px / 400 | 1.55, max 70–80ch |
| ui (buttons/labels) | 14px / 600 | 1.3 |
| meta | 12.5px / 500 | 1.4 |

### Spacing
Stock Tailwind scale only — 4/8/12/16/24/32/48px. No custom values.

### Radius & elevation
- `0px` — listing cards (no radius, no shadow; the photo carries the depth)
- `2px` — fields, buttons, most cards/panels
- `6px` + shadow `0 10px 26px -8px rgba(22,35,61,.26), 0 2px 6px -2px rgba(22,35,61,.14)` — **floating elements only**: modal, dropdown menu, toast. This is the one shadow in the whole product, tinted ink not neutral grey.

### Motion
120ms ease-out for hover border/fill changes only (no lift/scale/translate on hover). 200ms ease-out for floating elements entering. Exactly one orchestrated animation in the whole product: a 320ms brass-stamp confirmation on OTP success (see auth screen 2e). Everything collapses to 0ms under `prefers-reduced-motion`.

### Signature device — the dimension rule
A 1px `rule-400` line with two 9px perpendicular end-ticks, styled like an architectural dimension line. Exactly two permitted placements: a 120px short lead-in directly under a page title, or full-width as a section separator. Never both, never a third use.

## Screens
All screens are documented as labelled options inside the two bundled files — open the file, find the id (e.g. `2h`) to see the exact markup/CSS.每 screen below names its id(s) and file.

**Foundations + Public feed** — `My House - Plan & Laiton.dc.html`
- `1a` Full token sheet (palette, type scale, spacing, radius, elevation, motion, dimension-rule spec)
- `1b`/`1c` Public feed, desktop 1180 / mobile 360 — search bar, grid of listing cards, pagination, empty state, loading skeleton, error state
- `1f` **Approved** listing card: photo-bled brass price plate anchored into the photo like door hardware; "Verified" is a white label + brass dot on the card (full brass badge reserved for listing detail only)
- `1g` **Approved** public header: single row, 5 tabs (Home/Listings/Search/About/Contact) + Log in + brass "Post a property" CTA
- `1j` **Approved** dimension rule: 120px lead-in under a title, no result-count on this line (count lives in the filter row instead)
- `1d`/`1e`/`1h`/`1i`/`1k`/`1l` — discarded alternates, kept only for the record

**All other screens** — `My House - Screens.dc.html`
- Section 2 — Headers by role: `2a` seeker w/ pending-owner-request badge, `2b` owner w/ verified chip + brass "Add a property", `2c` admin (ink header, internal-tool tabs, no marketing chrome)
- Section 3 — Auth, 3 real steps: `2d` email entry (+ interrupted-registration notice), `2e` OTP desktop (default/invalid/rate-limited/success-with-stamp states), `2f` OTP mobile 360, `2g` profile completion (new users only)
- Section 4 — Listing detail: `2h` desktop w/ contact revealed + DET-01 warning `Alert`, `2i` mobile 360 w/ contact not yet revealed, `2j` gone/unavailable/failed-to-load states
- Section 5 — Owner property management: `2k` "My properties" list w/ availability toggle, `2l` create/edit form, `2m` photo upload w/ auto-cover-on-first-upload
- Section 6 — Owner role request: `2n` the single atomic form (identity + documents), `2o` pending/approved/rejected status views
- Section 7 — Admin back-office: `2p` owner-request queue + document viewer + approve/reject, `2q` users table w/ suspend/delete confirm, `2r` properties table w/ moderation actions
- Section 8 — Full prop/variant spec for the 6 proposed shared components (see below)
- Section 9 — Profile & account deletion: `3a` desktop, `3b` mobile 360, `3c` delete-account confirm modal
- Section 10 — Toast & modal inventory: `4a` four semantic toast tones, `4b` "Report this listing" modal (closes the loose end left by `2h`'s report link)

## Proposed new shared components
None of these exist in `frontend/src/shared/components/` yet — full spec (all props, types, defaults) is in section 8 of `My House - Screens.dc.html`. Summary:
- **SiteHeader** — `role: "public"|"seeker"|"owner"|"admin"` (required), `user`, `ownerRequestStatus`, `verified`, `pendingRequestCount`
- **SiteFooter** — `variant: "public"|"none"`
- **Badge** — `tone: "neutral"|"success"|"warning"|"error"|"brass"` (required), `dot`, `size`
- **DimensionRule** — `width: number|"full"` (default 120)
- **EmptyState** — `title` (required), `description`, `primaryAction`, `secondaryAction`
- **Skeleton** — `variant: "card"|"line"|"block"` (required), `aspectRatio`

## Interactions & behavior
- **OTP**: 6 boxed digits, auto-advance focus, clears + refocuses first box on invalid/expired code, resend countdown from `Retry-After` header (fallback 60s), the one orchestrated 320ms brass-stamp animation plays on success then navigates on.
- **Photo upload**: first successfully uploaded photo becomes the cover automatically (brass-framed tile + filled brass "COVER" tag); every other tile shows a persistent (not hover-only) "Make cover" affordance; over-size files show inline error, not a rejection dialog.
- **Listing availability toggle**: switching to unavailable immediately desaturates the row's thumbnail and greys its price in the owner's own list — visual confirmation of what the public no longer sees.
- **Contact reveal**: the DET-01 safety warning is shown to a seeker even before the phone number is revealed (mobile), and remains pinned directly under the number once revealed (desktop) — never collapsed into fine print.
- **Admin decisions**: Approve is the *only* solid-filled semantic button in the product — reserved because it's the one action here with an immediate, public, irreversible effect (grants the owner role, makes phone public). Reject requires a note; the note is shown verbatim on the applicant's status screen (2o).
- **Destructive confirmation** (delete user, delete own account): always the same modal shell — type the literal word "DELETE" to confirm, no plain second-click confirm anywhere in the product.
- **Toasts**: success/info auto-dismiss ~4s; warning/error persist until manually closed. Bottom-left on desktop, bottom-center above the tab bar on mobile.

## State / data notes
- `formatPrice` renders FCFA with no symbol prefix, unit appended in the view layer — confirmed as the single MVP currency for all listings, including French ones (no per-listing currency field for MVP).
- Owner's photo list has an explicit "cover" pointer, not just "first uploaded" — the UI must let the owner reassign it later (`Make cover`).
- `is_new_user` flag gates whether step 3 (profile completion) of auth renders at all.

## Explicitly out of scope this round
- **About / Contact / Search pages** — nav items exist as bare, unlinked `<a>` tags; no page designs, no `router.tsx` entries. Build these later if/when scoped.
- Language toggle, in-app messaging, favorites, interactive map — out of MVP scope per the brief.

## Assets
Listing photos in the mockups are placeholder stock (`assets/room-0X.webp`) explicitly flagged in the design as **not representative** — the brief calls for real, modest-to-mid-range photography from French and West African contexts. Do not ship the placeholder images; source real property photos before launch.

## Files in this bundle
- `My House - Plan & Laiton.dc.html` — design tokens + public feed (approved: card `1f`, header `1g`, dimension rule `1j`)
- `My House - Screens.dc.html` — all remaining screens (sections 2–10, ids `2a`–`4b`)
- `github.md` — source repo (`Rolland0000/My-House`, branch `develop`) and a table mapping each screen back to the repo files it's grounded in
