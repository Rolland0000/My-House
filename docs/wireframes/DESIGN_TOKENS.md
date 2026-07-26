# MyHouse — Design Tokens (MH-17)

Minimal token set derived from the low-fidelity wireframes (`mh-12-auth-flow.html`,
`mh-13-feed-detail.html`, `mh-14-listing-management.html`, `mh-15-owner-request.html`), which
already share one consistent CSS custom-property palette. Feeds directly into the Tailwind
config in MH-18 — no separate design-token tooling at this scale.

## Color

| Token | Light | Dark | Usage |
|---|---|---|---|
| `color-bg` | `#f5f4f0` | `#1c1b18` | page background |
| `color-surface` | `#ffffff` | `#26241f` | cards, inputs, panels |
| `color-text` | `#201f1c` | `#ece8df` | primary text |
| `color-text-muted` | `#57544c` | `#a8a297` | secondary/meta text |
| `color-border` | `#cdc9bf` | `#43403a` | default borders |
| `color-border-strong` | `#a39d8e` | `#5b574d` | emphasized borders, dashed dropzones |
| `color-primary` | `#a8380f` | `#e2794a` | brand, CTAs, links |
| `color-primary-soft` | `#f0ded4` | `#3a2a1f` | primary tint backgrounds |
| `color-success` | `#3f6b2e` | `#8fbf6d` | approved/validated states |
| `color-success-soft` | `#dfe8d5` | `#223a1c` | success tint backgrounds |
| `color-warning` | `#8a5a00` | `#d9ab4a` | pending/attention states |
| `color-warning-soft` | `#f2e4c4` | `#3a2f16` | warning tint backgrounds |
| `color-error` | `#a3241d` | `#e08076` | errors, rejected states |
| `color-error-soft` | `#f3dcd9` | `#3a2321` | error tint backgrounds |
| `color-focus` | `#1d4ed8` | `#6f9bff` | focus rings only |

`color-success` / `color-success-soft` have no equivalent in the wireframes (the wireframe
`--accent` doubles as a mockup-annotation "built" marker, not a semantic success state) — new
values chosen to match the desaturation level of the existing warning/error pair.

## Typography

- `font-sans` — `-apple-system, "Segoe UI", "Helvetica Neue", Arial, sans-serif` — body/UI text
- `font-mono` — `ui-monospace, "SF Mono", "Cascadia Code", "Roboto Mono", Consolas, monospace` —
  eyebrows, status tags, ticket references

| Token | Size | Usage |
|---|---|---|
| `text-sm` | 12px | labels, meta text, captions |
| `text-base` | 14px | body copy (collapses the wireframes' 13.5–14.5px range) |
| `text-lg` | 24px | page headings (rounds the wireframes' 26px) |

Capped at 3 sizes for MVP. The wireframes use a wider range (10–26px) because they are
detailed mockups, not tokens — anything outside this scale in a wireframe is incidental detail,
not a token to preserve.

Weights: `400` (body), `600` (labels, buttons), `700` (headings, strong emphasis).

## Spacing

No custom scale — reuse Tailwind's default 4px-based spacing scale directly. Every spacing
value used consistently across the wireframes (4/8/12/16/24/32/48px) already lands on a stock
Tailwind step. Semantic convention for MH-18:

| Alias | Value | Tailwind step |
|---|---|---|
| `xs` | 4px | `1` |
| `sm` | 8px | `2` |
| `md` | 12px | `3` |
| `lg` | 16px | `4` |
| `xl` | 24px | `6` |
| `2xl` | 32px | `8` |
| `3xl` | 48px | `12` |

## Border radius

| Token | Value | Usage |
|---|---|---|
| `radius-sm` | 4px | inputs, chips, small controls (rounds the wireframes' 3px) |
| `radius-md` | 8px | cards, panels, containers (rounds the wireframes' 6px) |
| `radius-full` | 9999px | pills, badges, avatars |
