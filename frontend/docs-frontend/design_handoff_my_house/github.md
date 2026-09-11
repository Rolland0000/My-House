repo: Rolland0000/My-House
branch: develop
path: frontend/src, docs/wireframes

## Last sync

date: 2026-09-08T15:32:00Z

### Updated in this project

- Step 2 delivered: all screen sets from the brief (headers by role, auth flow, listing detail, owner management, owner request, admin back-office).
- Screens grounded in the real component contracts (Button, Card, Alert, Input, Select, FormField, FileDropzone, Modal, Pagination) — props treated as frozen.
- Copy is English and prices are FCFA, per the client's step-2 direction. Confirmed: FCFA stays the single MVP currency everywhere, including French listings.
- Repo tokens (cream + terracotta) replaced token-for-token by "Plan & Brass" in `@theme`; no Tailwind class renames.
- Full props/variants spec added for the 6 proposed shared components (SiteHeader, SiteFooter, Badge, DimensionRule, EmptyState, Skeleton) — none exist in the repo yet.
- About/Contact stay as bare, unlinked nav items — no pages, no router.tsx entries built (deliberately out of scope for now).

## Screen map

| Screen (project)                  | Repo files                                                                                                                                                         |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Foundations "Plan & Brass"        | docs/wireframes/DESIGN_TOKENS.md, frontend/src/index.css                                                                                                           |
| Public feed + header/footer       | frontend/src/features/listings/components/ListingFeed.tsx, ListingCard.tsx, labels.ts, frontend/src/app/layout/RootLayout.tsx, frontend/src/shared/utils/format.ts |
| Headers by role (2a–2c)           | frontend/src/app/layout/RootLayout.tsx, AdminLayout.tsx, router.tsx, frontend/src/features/auth/AuthContext.tsx                                                    |
| Auth flow (2d–2g)                 | frontend/src/features/auth/components/AuthFlow.tsx, OtpVerifyForm.tsx, OtpCodeInput.tsx, RegistrationForm.tsx, frontend/src/features/auth/constants.ts             |
| Listing detail (2h–2j)            | frontend/src/features/listings/components/ListingDetail.tsx, frontend/src/shared/components/Alert.tsx                                                              |
| Owner property management (2k–2m) | frontend/src/shared/components/FileDropzone.tsx, FormField.tsx, TextArea.tsx, frontend/src/features/listings/api.ts                                                |
| Owner role request (2n–2o)        | frontend/src/shared/components/FileDropzone.tsx, Alert.tsx, frontend/src/features/profile/profileValidation.ts                                                     |
| Admin back-office (2p–2r)         | frontend/src/app/layout/AdminLayout.tsx, frontend/src/shared/components/Modal.tsx, Pagination.tsx                                                                  |
| Proposed component specs (t8)     | frontend/src/shared/components/ (target dir, files not yet created)                                                                                                |
