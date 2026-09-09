# MyHouse — Design Tokens ("Plan & Brass")

Token set for the "Plan & Brass" visual identity — ink/brass palette, Archivo type,
the dimension-rule graphic device — delivered as a high-fidelity design handoff in
`frontend/docs-frontend/design_handoff_my_house/` and applied token-for-token in
`frontend/src/index.css`'s `@theme` block. Supersedes the original MH-17 minimal
cream/terracotta set derived from the low-fidelity wireframes.

## Color

| Token | Hex | Usage |
|---|---|---|
| `ink-900` | `#16233D` | Headings, dark section backgrounds (footer, admin header) |
| `ink-600` | `#2E4A73` | Links, secondary/outline buttons, focus ring |
| `ink-500` | `#55606F` | Metadata/secondary text (5.4:1 on white) |
| `brass-500` | `#BE8A2E` | Single accent — primary CTA, verified seal, price plate border. Label text on brass is always `ink-900`, never white (white-on-brass is 2.4:1, under AA) |
| `brass-600` | `#9C7327` | Brass hover/active + border under brass fills |
| `paper-50` | `#F4F5F2` | Page background |
| `text-900` | `#1A1D1B` | Body text (15.8:1 on paper) |
| `rule-300` | `#D8DAD4` | Default 1px border (cards, fields, table rows) |
| `rule-400` | `#BDC1BA` | Hover border, dimension-rule line, dropzone border |
| success | `#2F6E4E` (icon/text) on `#EAF0EC` fill | Approved / available / published |
| warning | icon `#C77D1D`, on-fill text `#8F5A12`, fill `#F8F1E5` | Pending / caution |
| error | icon `#B94A3D`, on-fill text `#9A3A2E`, fill `#F7EBE9` | Rejected / destructive / failure |
| info | `#2E4A73` (ink-600) on `#EEF1F5` fill | Neutral notices |

Cards are pure white `#FFFFFF` on `paper-50` — depth comes from that contrast, not
shadow. Warning/error on-fill text steps one shade darker than the icon color to
hold 4.5:1 at body size.

No dark mode: the palette is a light, paper/ink aesthetic by definition, so the
previous `prefers-color-scheme: dark` override block has been removed.

## Typography

Single family: **Archivo** (variable, weights 400/500/600/700/800), loaded via
Google Fonts in `frontend/index.html`. Picked partly for French diacritics, so no
type change is needed when the app ships French copy.

| Role | Size/weight | Line-height |
|---|---|---|
| display | 34px / 800 | 1.12, letter-spacing -.015em |
| h1 | 26px / 700 | 1.2 |
| h2 | 19px / 700 | 1.25 |
| body | 15px / 400 | 1.55, max 70–80ch |
| ui (buttons/labels) | 14px / 600 | 1.3 |
| meta | 12.5px / 500 | 1.4 |

## Spacing

Stock Tailwind scale only — 4/8/12/16/24/32/48px. No custom values.

## Radius & elevation

- `0px` (`rounded-none`) — listing cards only. No radius, no shadow; the photo
  carries the depth.
- `2px` (`rounded-sm`) — fields, buttons, and most cards/panels.
- `6px` + `shadow-elevated` (`rounded-lg`) — floating elements only: modal,
  dropdown menu, toast. `--shadow-elevated: 0 10px 26px -8px rgba(22,35,61,.26),
  0 2px 6px -2px rgba(22,35,61,.14)` — the one shadow in the whole product, tinted
  ink rather than neutral grey.

## Motion

120ms ease-out for hover border/fill changes only (no lift/scale/translate on
hover). 200ms ease-out for floating elements entering. Exactly one orchestrated
animation in the whole product: a 320ms brass-stamp confirmation on OTP success
(`--animate-mh-stamp` in `index.css`). Everything collapses to ~0ms under
`prefers-reduced-motion` (enforced globally in `index.css`, not per-component).

## The dimension rule

Signature device: a 1px `rule-400` line with two 9px perpendicular end-ticks,
styled like an architectural dimension line (`shared/components/DimensionRule.tsx`).
Exactly two permitted placements — a 120px short lead-in directly under a page
title, or full-width as a section separator. Never both, never a third use.
