# Graph Report - My-House  (2026-10-03)

## Corpus Check
- 322 files · ~170,441 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 24 file(s) not represented in the graph (top: (none) 15, .toml 5, .example 2)

## Summary
- 2464 nodes · 5482 edges · 190 communities (120 shown, 70 thin omitted)
- Extraction: 95% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 243 edges (avg confidence: 0.91)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c1baa424`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- listings/service.rs
- owner_requests/dto.rs
- auth/index.ts
- config/mod.rs
- owner_requests/service.rs
- listings/dto.rs
- AppCacheProvider
- validate_listing
- components/index.ts
- Mailer
- auth/repository.rs
- MyHouse Project Instructions (Agents)
- jwt.rs
- local_fs.rs
- users/repository.rs
- listings/index.ts
- extractors.rs
- validation.rs
- EditListingPage.tsx
- users/service.rs
- admin/api.ts
- AppState
- compilerOptions
- Code Review — Backend Skill
- owner_requests/repository.rs
- devDependencies
- Module: users
- owner_requests/model.rs
- media/handler.rs
- compilerOptions
- auth/handler.rs
- 20260729041038_baseline_schema.sql
- ListingForm.tsx
- media/service.rs
- admin/handler.rs
- .mcp.json
- ProfileForm.tsx
- Design Handoff README
- AppError
- What You Must Do When Invoked
- pagination.rs
- users/handler.rs
- dependencies
- MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)
- admin identity-document read exception
- ListingSummaryRow
- owner_requests/handler.rs
- rate_limit.rs
- scripts
- owner-request/api.ts
- client.ts
- media/repository.rs
- errors.rs
- CI Backend Workflow
- app_server.rs
- api/constants.ts
- OtpVerifyForm.tsx
- sqlx::query! / query_as! compile-time checked queries
- health.rs
- users table
- mh-15-owner-request.html wireframe
- MH-53 — Créer un bien : plan d'implémentation (BE + FE)
- backend-dev service
- rbac.rs
- find_document_entry
- listings/handler.rs
- Docker Rules Skill
- Toast.tsx
- listing_media table
- package.json
- listings/repository.rs
- CI Job: Frontend OpenAPI Codegen
- CLAUDE.md — MyHouse project instructions
- ON DELETE CASCADE — relational-only cleanup rule
- @tanstack/react-query
- graphify reference: extra exports and benchmark
- users module
- shared module
- Auth API endpoints (§4.1)
- Owner Request Rejected Email Template
- Embedded React/TypeScript Rules Content
- graphify reference: extra exports and benchmark
- MH-57 — Changer le statut d'un bien : plan d'implémentation (BE + FE)
- db.rs
- What You Must Do When Invoked
- OtpCodeInput
- route.rs
- owner_requests table
- ErrorBody
- router.tsx
- sqlx
- graphify reference: query, path, explain
- mh-12-auth-flow.html wireframe
- analyze job (rust + javascript-typescript matrix)
- Root Docker Compose
- pre-tool-use.sh
- Registration Ticket (opaque UUID)
- mh-13-feed-detail.html wireframe
- tsconfig.json
- README.md — Project Overview and Setup
- graphify reference: query, path, explain
- logging.rs
- AuthUser Extractor
- No-Proxy Principle for Public Files
- R-08: search_vector trigger N+1 query
- Refresh Token httpOnly Cookie
- GET /users/me/owner-request
- RateLimitState
- labels.ts
- OwnerListings.tsx
- RootLayout.tsx
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- pre-commit
- graphify Slash Command Trigger (.claude/CLAUDE.md)
- Backend Review Checklist Reference
- AppError Centralized Error Type
- Graceful Shutdown (SIGTERM)
- GET /health Endpoint
- Handler-Service-Repository Layering
- Modular Monolith
- OpenAPI-driven TypeScript Type Generation
- Pagination Standard
- R-07: No index on listings.price
- Admin API endpoints (§4.7)
- GET /listings/:id/contact (§4.6)
- contact module
- fn_set_updated_at trigger fn
- Listings API endpoints (§4.3)
- notifications module
- GET /search (§4.4)
- Users API endpoints (§4.2)
- Modern Villa Night Exterior with Pool (OIP 10)
- Bedroom Interior Reference Photo (OIP 11)
- Green Bedroom Design Reference Photo
- Wood-Paneled Bedroom with Panoramic Ocean View
- OIP (3) - House with Pool Rendering
- Bedroom Interior Reference Photo (OIP 4)
- Bedroom with Garden Deck Access (Stock Photo)
- Modern Villa with Pool Stock Photo (OIP 6)
- Modern Villa with Pool at Dusk (OIP 7)
- Rustic Bedroom Reference Photo (OIP 8)
- Bedroom Interior Photo (OIP 9)
- OIP Bedroom Design Reference Photo
- graphify reference: incremental update and cluster-only
- vite.config.ts
- file_validation.rs
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- overrides
- .claude/skills/graphify/references/extraction-spec.md
- .codex/skills/graphify/references/extraction-spec.md
- backend-my-house
- OwnerListingRow.tsx
- vitest
- OwnerRequestDetail.tsx
- super
- 2.3 MH-54-FE — Fichiers, dans l'ordre (un fichier à la fois)
- main.rs
- rate_limit

## God Nodes (most connected - your core abstractions)
1. `AppError` - 189 edges
2. `AppState` - 68 edges
3. `cn()` - 46 edges
4. `react` - 41 edges
5. `AppCacheProvider` - 32 edges
6. `AuthUser` - 30 edges
7. `set_valid_env()` - 29 edges
8. `react-router` - 25 edges
9. `Alert()` - 25 edges
10. `Mailer` - 24 edges

## Surprising Connections (you probably didn't know these)
- `9. Critères de fin` --references--> `updateListingStatus()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/api.ts
- `3.2 Fichiers, dans l'ordre (un fichier à la fois)` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-53_IMPLEMENTATION_PLAN.md → frontend/src/features/listings/components/ListingNotFound.tsx
- `Décisions prises` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/components/ListingNotFound.tsx
- `3. Ordre d'implémentation recommandé` --references--> `OwnerBar()`  [INFERRED]
  docs/plans/MH-54-plan.md → frontend/src/features/listings/components/OwnerBar.tsx
- `8. Hors périmètre` --references--> `OwnerBar()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/components/OwnerBar.tsx

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Frontend CI Pipeline Gate Sequence** — github_workflows_ci_frontend_frontend_install, github_workflows_ci_frontend_frontend_codegen, github_workflows_ci_frontend_frontend_lint, github_workflows_ci_frontend_frontend_test, github_workflows_ci_frontend_frontend_build [EXTRACTED 0.90]
- **Proposed Shared Component Library Spec** — frontend_docs_frontend_design_handoff_my_house_readme_siteheader, frontend_docs_frontend_design_handoff_my_house_readme_sitefooter, frontend_docs_frontend_design_handoff_my_house_readme_badge, frontend_docs_frontend_design_handoff_my_house_readme_dimensionrule, frontend_docs_frontend_design_handoff_my_house_readme_emptystate, frontend_docs_frontend_design_handoff_my_house_readme_skeleton [EXTRACTED 0.90]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **Explicit-sequencing cleanup pattern for account/file deletion** — claude_rules_database_cascade_cleanup, claude_rules_database_storageprovider_delete, claude_rules_database_architecture_md [EXTRACTED 1.00]
- **Safe, checked query construction pattern** — claude_rules_database_query_macros, claude_rules_database_querybuilder, claude_rules_database_sql_injection_prevention [EXTRACTED 1.00]
- **MyHouse Domain Modules (Modular Monolith)** — docs_architecture_v1_2_module_auth, docs_architecture_v1_2_module_users, docs_architecture_v1_2_module_listings, docs_architecture_v1_2_module_search, docs_architecture_v1_2_module_media, docs_architecture_v1_2_module_contact, docs_architecture_v1_2_module_notifications, docs_architecture_v1_2_module_admin [EXTRACTED 1.00]
- **MVP notification delivery pipeline** — docs_technical_spec_mvp_v1_2_users_module, docs_technical_spec_mvp_v1_2_admin_module, docs_technical_spec_mvp_v1_2_notifications_service_send_email, docs_technical_spec_mvp_v1_2_infra_mailer, docs_technical_spec_mvp_v1_2_env_vars [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin [EXTRACTED 1.00]
- **OTP Login/Registration Flow** — docs_architecture_v1_2_otp_passwordless_auth, docs_architecture_v1_2_registration_ticket, docs_architecture_v1_2_atomic_registration [EXTRACTED 1.00]
- **Owner Upgrade Request/Validation Workflow** — docs_architecture_v1_2_owner_request_flow, docs_architecture_v1_2_admin_bootstrap, docs_technical_spec_mvp_v1_2_owner_requests_table [EXTRACTED 1.00]
- **storage key naming convention (public vs private prefixes)** — docs_technical_spec_mvp_v1_2_public_listing_storage_key, docs_technical_spec_mvp_v1_2_public_avatar_storage_key, docs_technical_spec_mvp_v1_2_private_owner_request_storage_key, docs_technical_spec_mvp_v1_2_storageprovider_trait [EXTRACTED 1.00]
- **Notifications Module Email Template Set** — backend_src_modules_notifications_templates_otp, backend_src_modules_notifications_templates_welcome, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.80]
- **Shared Four-Phase Review Structure** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_code_review_backend_skill_four_phase_review_process, claude_skills_code_review_frontend_skill_four_phase_review_process [INFERRED 0.85]
- **Dev Environment Docker Compose Stack** — backend_compose_backend_backend_dev, frontend_compose_frontend_frontend_dev, docker_compose_db, docker_compose_mailhog [INFERRED 0.85]
- **Dimension Rule Signature Device Across Design Docs** — docs_wireframes_design_tokens_dimension_rule, frontend_docs_frontend_design_handoff_my_house_readme_dimensionrule, frontend_docs_frontend_design_handoff_my_house_my_house_plan_laiton_dc [INFERRED 0.85]
- **MVP security/data-integrity invariants** — docs_technical_spec_mvp_v1_2_magic_bytes_validation, docs_technical_spec_mvp_v1_2_server_generated_filenames, docs_technical_spec_mvp_v1_2_owner_request_atomic_submission, docs_technical_spec_mvp_v1_2_refresh_token_rotation [INFERRED 0.85]
- **Skills Deferring to ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_github_ticket_skill_github_ticket, docs_architecture_doc, docs_technical_spec_mvp_doc [INFERRED 0.85]
- **MyHouse Locked Decisions Enforced Across Project Instructions and Review Skills** — agents_rules_insrtruction_for_my_house_key_decisions_locked, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Owner Request Approval Notification Flow** — backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.85]
- **Refresh Token Handling Invariants Across Review Skills** — claude_skills_code_review_backend_skill_refresh_token_rotation_invariant, claude_skills_code_review_frontend_skill_refresh_token_cookie_invariant, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]

## Communities (190 total, 70 thin omitted)

### Community 0 - "listings/service.rs"
Cohesion: 0.12
Nodes (22): a_valid_request_is_normalized(), an_unknown_type_is_rejected_and_every_label_is_accepted(), description_bounds(), DESCRIPTION_LENGTH, each_required_field_missing_is_reported(), optional_fields_may_be_absent(), PLACE_NAME_MAX_LENGTH, price_bounds() (+14 more)

### Community 1 - "owner_requests/dto.rs"
Cohesion: 0.12
Nodes (29): admin_detail_dto_falls_back_to_empty_documents_on_malformed_json(), admin_detail_dto_never_exposes_storage_key(), admin_detail_row(), admin_list_dto_omits_identity_data_and_documents(), admin_row(), AdminOwnerRequestDetailDto, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocumentDto (+21 more)

### Community 2 - "auth/index.ts"
Cohesion: 0.14
Nodes (22): OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken, RefreshResponse, refreshSession(), registerAccount(), RegisterPayload, RegisterResponse (+14 more)

### Community 3 - "config/mod.rs"
Cohesion: 0.07
Nodes (67): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, dev_allows_smtp_credentials_over_plaintext(), dev_without_smtp_credentials_sends_plaintext_unauthenticated(), empty_smtp_credentials_count_as_absent() (+59 more)

### Community 4 - "owner_requests/service.rs"
Cohesion: 0.10
Nodes (36): a_missing_storage_object_surfaces_as_document_not_found(), a_pdf_renamed_into_an_image_slot_is_a_shape_violation(), accepts_one_pdf_with_no_side(), accepts_two_images_as_front_and_back(), as_document_not_found(), as_shape_error(), ClassifiedDocument, classify_documents() (+28 more)

### Community 5 - "listings/dto.rs"
Cohesion: 0.23
Nodes (21): a_draft_summary_serializes_published_at_as_null(), a_published_summary_keeps_its_published_at(), ListingDetailDto, ListingMediaDto, ListingRequest, ListingStatusDto, ListingStatusRequest, ListingStatusResponse (+13 more)

### Community 6 - "AppCacheProvider"
Cohesion: 0.05
Nodes (68): async_trait, AppCache, AUTH_CHALLENGE_MAX_ENTRIES, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache() (+60 more)

### Community 7 - "validate_listing"
Cohesion: 0.21
Nodes (20): an_unknown_or_missing_status_is_a_status_field_error(), create_listing(), get_listing_detail(), is_visible_to(), list_listings(), list_owner_listings(), parse_status(), ListingRequest (+12 more)

### Community 8 - "components/index.ts"
Cohesion: 0.05
Nodes (52): AlertProps, AlertVariant, variantConfig, Badge(), BadgeProps, BadgeSize, sizeClasses, toneClasses (+44 more)

### Community 9 - "Mailer"
Cohesion: 0.05
Nodes (54): Address, AddressError, AsyncSmtpTransport, authenticates_with_configured_credentials(), builds_successfully_with_valid_config(), builds_with_credentials_and_each_security_mode(), credentials(), hello_name() (+46 more)

### Community 10 - "auth/repository.rs"
Cohesion: 0.21
Nodes (20): NewAccount, RefreshTokenLookup, Option, Role, Uuid, create_account(), db_err(), email_exists() (+12 more)

### Community 11 - "MyHouse Project Instructions (Agents)"
Cohesion: 0.06
Nodes (36): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+28 more)

### Community 12 - "jwt.rs"
Cohesion: 0.09
Nodes (37): Algorithm, hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest(), String, sha256_hex(), Claims, encode_with_exp() (+29 more)

### Community 13 - "local_fs.rs"
Cohesion: 0.15
Nodes (25): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, OWNER_REQUESTS_PREFIX, presigned_url_returns_not_implemented_error(), read_on_missing_key_returns_not_found_not_generic_storage_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes() (+17 more)

### Community 14 - "users/repository.rs"
Cohesion: 0.37
Nodes (15): admin_exists(), delete_by_id(), find_by_id(), find_is_active(), list_listing_media_keys_for_owner(), list_owner_request_document_keys(), Option, PgPool (+7 more)

### Community 15 - "listings/index.ts"
Cohesion: 0.19
Nodes (20): 4. Points d'intégration avec l'existant, ListingFeed, getListing(), ListingDetail, ListingSummary, listListings(), ListListingsParams, listOwnerListings() (+12 more)

### Community 16 - "extractors.rs"
Cohesion: 0.11
Nodes (29): AppJson<T>, AuthState, AuthUser, bearer_token(), MaybeAuthUser, optional_bearer_token(), resolve_identity(), resolve_identity_rejects_an_undecodable_token_without_touching_cache_or_db() (+21 more)

### Community 17 - "validation.rs"
Cohesion: 0.08
Nodes (24): FieldErrors, finish_reports_one_entry_per_recorded_violation(), MAX_ADMIN_NOTE_LENGTH, MAX_NAME_LENGTH, MAX_PHONE_LENGTH, normalize_place_name(), NUL, optional_name() (+16 more)

### Community 18 - "EditListingPage.tsx"
Cohesion: 0.13
Nodes (27): 1. Contexte et objectif, 2.1 MH-56-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-56-BE — Tests unitaires (logique pure, sans DB), 2.3 MH-56-FE — Fichiers, dans l'ordre (un fichier à la fois), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant, 5. Vérifications (+19 more)

### Community 19 - "users/service.rs"
Cohesion: 0.05
Nodes (61): AvatarUploadForm, response_envelope_serializes_the_profile_fields(), row(), row_maps_to_dto_field_for_field(), From, Option, Role, Self (+53 more)

### Community 20 - "admin/api.ts"
Cohesion: 0.16
Nodes (20): AdminOwnerRequest, AdminOwnerRequestDetail, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocument, getOwnerRequest(), getOwnerRequestDocument(), listOwnerRequests(), ListOwnerRequestsParams (+12 more)

### Community 21 - "AppState"
Cohesion: 0.11
Nodes (27): AppState, Inner, Arc, PgPool, Self, StorageProvider, router(), OpenApiRouter (+19 more)

### Community 22 - "compilerOptions"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+11 more)

### Community 23 - "Code Review — Backend Skill"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 24 - "owner_requests/repository.rs"
Cohesion: 0.28
Nodes (17): count_for_admin(), db_err(), find_by_id_for_admin(), find_current_for_user(), insert_pending(), list_for_admin(), pending_exists_for_user(), review_for_admin() (+9 more)

### Community 25 - "devDependencies"
Cohesion: 0.11
Nodes (18): devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, globals, openapi-typescript, prettier (+10 more)

### Community 26 - "Module: users"
Cohesion: 0.13
Nodes (20): Account Deletion Cascade + Storage Cleanup, Single Admin Account Bootstrap, AwsS3Storage (V2), LocalFsStorage, Module: admin, Module: auth, Module: contact, Module: listings (+12 more)

### Community 27 - "owner_requests/model.rs"
Cohesion: 0.26
Nodes (18): AdminOwnerRequestDetailRow, AdminOwnerRequestRow, IdentityData, OwnerRequestDocument, OwnerRequestRow, OwnerRequestStatus, OwnerRequestSubmission, Bytes (+10 more)

### Community 28 - "media/handler.rs"
Cohesion: 0.19
Nodes (18): delete_media(), FILE_FIELD_NAME, LISTING_ID_FIELD_NAME, multipart_error(), promote_cover(), read_submission(), Json, Multipart (+10 more)

### Community 29 - "compilerOptions"
Cohesion: 0.11
Nodes (17): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+9 more)

### Community 30 - "auth/handler.rs"
Cohesion: 0.21
Nodes (19): logout(), otp_request(), otp_verify(), refresh(), register(), CookieJar, Json, Result (+11 more)

### Community 31 - "20260729041038_baseline_schema.sql"
Cohesion: 0.13
Nodes (25): idx_listing_media_listing, idx_listing_media_one_cover, idx_listings_city, idx_listings_city_normalized, idx_listings_owner, idx_listings_search, idx_listings_status, idx_listings_type (+17 more)

### Community 32 - "ListingForm.tsx"
Cohesion: 0.11
Nodes (34): 4.3 Tests unitaires (vitest, logique pure), 2.4 MH-56-FE — Tests unitaires (vitest, logique pure), 6. Risques et inconnues, ListingType, ListingForm(), onSubmit(), ListingFormMode, ListingFormProps (+26 more)

### Community 33 - "media/service.rs"
Cohesion: 0.05
Nodes (61): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+53 more)

### Community 34 - "admin/handler.rs"
Cohesion: 0.26
Nodes (15): AdminOwnerRequestDetailResponse, get_owner_request(), get_owner_request_document(), list_owner_requests(), review_owner_request(), IntoResponse, Json, Path (+7 more)

### Community 35 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 36 - "ProfileForm.tsx"
Cohesion: 0.11
Nodes (21): 2. Existant sur lequel on s'appuie, 4.2 Fichiers, dans l'ordre (un fichier à la fois), OwnerRequestForm, ProfileForm, FileWithPreview, FormValues, ID_TYPE_OPTIONS, ImageSlotProps (+13 more)

### Community 37 - "Design Handoff README"
Cohesion: 0.14
Nodes (16): MyHouse Design Tokens (Plan & Brass), Dimension Rule Signature Device, MH-17 Superseded Cream/Terracotta Palette, Design Handoff Repo Sync Log & Screen Map, My House - Plan & Laiton Design Mockup, My House - Screens Design Mockup, Design Handoff README, Admin Approve as Sole Solid Semantic Button (+8 more)

### Community 38 - "AppError"
Cohesion: 0.19
Nodes (13): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage (+5 more)

### Community 39 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 40 - "pagination.rs"
Cohesion: 0.15
Nodes (16): DEFAULT_PER_PAGE, MAX_PER_PAGE, PaginatedResponse<T>, PaginationMeta, Option, Self, T, Vec (+8 more)

### Community 41 - "users/handler.rs"
Cohesion: 0.20
Nodes (18): AVATAR_FIELD_NAME, delete_me(), get_me(), multipart_error(), read_file_field(), Bytes, CookieJar, Json (+10 more)

### Community 42 - "dependencies"
Cohesion: 0.25
Nodes (8): dependencies, clsx, lucide-react, react, react-dom, react-hook-form, react-router, @tanstack/react-query

### Community 43 - "MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)"
Cohesion: 0.20
Nodes (9): 1. Contexte et objectif, 3. Ordre d'implémentation recommandé, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE), 8. Hors périmètre, 9. Critères de fin, Décisions de conception prises pendant l'analyse (+1 more)

### Community 44 - "admin identity-document read exception"
Cohesion: 0.15
Nodes (14): admin identity-document read exception, ARCHITECTURE.md (referenced doc), backend/Dockerfile, docker-compose.yml, backend/.env.example variables, frontend/Dockerfile, GET /admin/owner-requests/:id/documents/:doc_id, infra/mailer.rs (lettre SMTP client) (+6 more)

### Community 45 - "ListingSummaryRow"
Cohesion: 0.23
Nodes (13): Self, Vec, ListingDetailRow, ListingFields, ListingMediaRow, ListingStatus, ListingSummaryRow, ListingType (+5 more)

### Community 46 - "owner_requests/handler.rs"
Cohesion: 0.18
Nodes (16): DOCUMENTS_FIELD_NAME, get_owner_request_status(), multipart_error(), read_submission(), Json, Multipart, MultipartError, Result (+8 more)

### Community 47 - "rate_limit.rs"
Cohesion: 0.13
Nodes (22): apperror, atomic, allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it(), distinct_clients_get_distinct_counters(), falls_back_to_peer_ip_when_trusted_header_is_missing(), headers_with_xff(), ignores_x_forwarded_for_from_an_untrusted_peer(), NGINX (+14 more)

### Community 48 - "scripts"
Cohesion: 0.15
Nodes (13): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+5 more)

### Community 49 - "owner-request/api.ts"
Cohesion: 0.17
Nodes (17): OwnerRequestStatus, BadgeSpec, getOwnerRequestStatus(), OwnerRequest, OwnerRequestResponse, ownerRequestStatusQueryKey, OwnerRequestStatusResponse, submitOwnerRequest() (+9 more)

### Community 50 - "client.ts"
Cohesion: 0.19
Nodes (18): AccessTokenGetter, apiDelete(), apiGetBlob(), apiPatch(), apiPost(), apiPut(), apiUpload(), buildQueryString() (+10 more)

### Community 51 - "media/repository.rs"
Cohesion: 0.36
Nodes (14): count_media(), db_err(), delete_media(), demote_other_covers(), find_listing_media(), insert_media(), lock_listing_owner(), lock_media_for_deletion() (+6 more)

### Community 52 - "errors.rs"
Cohesion: 0.26
Nodes (13): parse_envelope(), Response, StatusCode, Value, test_bad_request_carries_detail(), test_errors_other_than_validation_omit_fields(), test_internal_error_is_500(), test_listing_not_found_produces_correct_envelope() (+5 more)

### Community 53 - "CI Backend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 54 - "app_server.rs"
Cohesion: 0.18
Nodes (11): AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), Box, build_router (+3 more)

### Community 55 - "api/constants.ts"
Cohesion: 0.09
Nodes (34): OwnerRequestForm(), GENERIC_ERROR_MESSAGE, IMAGE_FORMAT_MESSAGE, IMAGE_SIZE_MESSAGE, PDF_FORMAT_MESSAGE, PDF_SIZE_MESSAGE, preCheckImage(), preCheckPdf() (+26 more)

### Community 56 - "OtpVerifyForm.tsx"
Cohesion: 0.12
Nodes (22): AuthFlow, requestOtp(), AuthFlow(), leaveRegistration(), AuthStepIndicator(), markInterrupted(), readInterrupted(), Screen (+14 more)

### Community 57 - "sqlx::query! / query_as! compile-time checked queries"
Cohesion: 0.22
Nodes (10): DATABASE_URL compile-time requirement, fn_set_updated_at trigger, Partial unique indexes for 'at most one active X' invariants, sqlx::query! / query_as! compile-time checked queries, sqlx QueryBuilder, Repository tests in rolled-back transaction (integration feature flag), Schema conventions (UUID PK, timestamps, enums), No string-interpolated SQL rule (+2 more)

### Community 58 - "health.rs"
Cohesion: 0.23
Nodes (9): axum, check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus (+1 more)

### Community 59 - "users table"
Cohesion: 0.24
Nodes (10): fn_cascade_owner_name_to_listings trigger fn, fn_update_listing_search_vector trigger fn, listing_status enum, listing_type enum, listings module, listings table, search module, user_role enum (+2 more)

### Community 60 - "mh-15-owner-request.html wireframe"
Cohesion: 0.20
Nodes (10): mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire'), Identity document dropzone component, Request approved status page (+2 more)

### Community 61 - "MH-53 — Créer un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.13
Nodes (14): 1. Décisions tranchées, 3.1 Contrat, 3.2 Fichiers, dans l'ordre (un fichier à la fois), 3.3 Tests unitaires (logique pure, sans DB), 3.4 Vérifications BE, 3. MH-53-BE — Création d'un bien, 4.1 Préalable, 4.4 Vérifications FE (+6 more)

### Community 62 - "backend-dev service"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 63 - "rbac.rs"
Cohesion: 0.24
Nodes (4): failed_check_maps_to_forbidden(), require_role(), Role, Result

### Community 64 - "find_document_entry"
Cohesion: 0.60
Nodes (5): a_doc_id_from_a_different_request_does_not_resolve(), documents_json(), find_document_entry(), finds_the_document_matching_doc_id_in_its_own_request(), Value

### Community 65 - "listings/handler.rs"
Cohesion: 0.21
Nodes (21): ListingDetailResponse, create(), get_by_id(), list(), list_mine(), OWNER_ROLES, Json, ListingRequest (+13 more)

### Community 66 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 67 - "Toast.tsx"
Cohesion: 0.18
Nodes (10): Providers(), ProvidersProps, queryClient, ToastContext, ToastContextValue, ToastItem, ToastOptions, ToastProvider() (+2 more)

### Community 68 - "listing_media table"
Cohesion: 0.25
Nodes (8): automatic cover photo selection, AwsS3Storage (V2), listing_media table, LocalFsStorage, magic-bytes file validation, Media API endpoints (§4.5), server-generated (UUID) filenames, StorageProvider trait

### Community 69 - "package.json"
Cohesion: 0.11
Nodes (19): name, private, type, version, clsx, eslint, @eslint/js, eslint-plugin-react-hooks (+11 more)

### Community 70 - "listings/repository.rs"
Cohesion: 0.22
Nodes (23): count_listings(), count_owner_listings(), find_listing_by_id(), find_media_for_listing(), insert_listing(), list_listings(), list_owner_listings(), LISTING_CURRENCY (+15 more)

### Community 71 - "CI Job: Frontend OpenAPI Codegen"
Cohesion: 0.43
Nodes (8): CI Job: Frontend Build, CI Job: Frontend OpenAPI Codegen, CI Job: Frontend Docker Build, CI Job: Frontend Install, CI Job: Frontend Lint, CI Job: Frontend Prettier, CI Job: Frontend Unit Tests, CI Job: Frontend TypeScript

### Community 72 - "CLAUDE.md — MyHouse project instructions"
Cohesion: 0.29
Nodes (7): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), mcp/.toolbox/tool.yaml — postgres-local MCP toolbox source

### Community 73 - "ON DELETE CASCADE — relational-only cleanup rule"
Cohesion: 0.29
Nodes (6): GET /admin/owner-requests/:id/documents/:doc_id, ARCHITECTURE.md, ON DELETE CASCADE — relational-only cleanup rule, listings.price missing index (known gap R-07), listings/search index checklist (idx_listings_*), Sensitive columns exclusion (identity_data, identity_documents JSONB)

### Community 74 - "@tanstack/react-query"
Cohesion: 0.26
Nodes (11): deleteAccount(), getMe(), profileQueryKey, updateMe(), UpdateProfilePayload, uploadAvatar(), UserResponse, useDeleteAccount() (+3 more)

### Community 75 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 76 - "users module"
Cohesion: 0.38
Nodes (7): admin module, media module, Notification stack (§3bis), notifications::service::send_xxx_email, synchronous best-effort email send, Testing strategy (§6), users module

### Community 77 - "shared module"
Cohesion: 0.33
Nodes (7): AppError, AuthUser extractor, frontend features/ directory, OpenAPI → TypeScript type generation pipeline, PaginatedResponse<T>, RBAC role guards, shared module

### Community 78 - "Auth API endpoints (§4.1)"
Cohesion: 0.33
Nodes (6): Auth API endpoints (§4.1), auth module, OTP passwordless auth, refresh token rotation, refresh_tokens table, registration_ticket

### Community 79 - "Owner Request Rejected Email Template"
Cohesion: 0.33
Nodes (6): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template, Optional rejection reason, Resubmission allowed after correction, user_name template variable

### Community 80 - "Embedded React/TypeScript Rules Content"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 81 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 82 - "MH-57 — Changer le statut d'un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.12
Nodes (16): 1. Contexte et objectif, 2.1 MH-57-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-57-BE : tests unitaires (logique pure, sans DB), 2.3 MH-57-FE : fichiers, dans l'ordre (un fichier à la fois), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications, 6. Risques et inconnues (+8 more)

### Community 83 - "db.rs"
Cohesion: 0.29
Nodes (7): connect_db(), MAX_CONNECTIONS, Error, PgPool, Result, sqlx::PgPool connection pooling, pgpooloptions

### Community 84 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 85 - "OtpCodeInput"
Cohesion: 0.57
Nodes (7): OtpCodeInput(), focusInput(), handleChange(), handleKeyDown(), handlePaste(), setDigit(), submit()

### Community 86 - "route.rs"
Cohesion: 0.19
Nodes (13): apidoc, appenv, ApiDoc, build_router(), merged_router(), openapi_spec(), build_cors_layer, modules (+5 more)

### Community 87 - "owner_requests table"
Cohesion: 0.40
Nodes (5): owner request atomic submission, owner_request_status enum, owner_requests table, PATCH /admin/owner-requests/:id, POST /owner-requests

### Community 88 - "ErrorBody"
Cohesion: 0.40
Nodes (6): ErrorBody, ErrorEnvelope, FieldError, Option, String, Vec

### Community 89 - "router.tsx"
Cohesion: 0.13
Nodes (21): Décisions de conception prises pendant l'analyse (sans question), 4. Points d'intégration avec l'existant, App(), AuthLayout(), RequireAdmin(), RequireAdminProps, RequireAuth(), RequireAuthProps (+13 more)

### Community 90 - "sqlx"
Cohesion: 0.50
Nodes (4): Forward-only migrations policy, No ORM — Prisma/Supabase rejected, PostgreSQL, sqlx

### Community 91 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 92 - "mh-12-auth-flow.html wireframe"
Cohesion: 0.50
Nodes (4): mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 93 - "analyze job (rust + javascript-typescript matrix)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 94 - "Root Docker Compose"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 96 - "Registration Ticket (opaque UUID)"
Cohesion: 0.67
Nodes (3): Atomic Registration via POST /auth/register, OTP Passwordless Authentication, Registration Ticket (opaque UUID)

### Community 97 - "mh-13-feed-detail.html wireframe"
Cohesion: 0.67
Nodes (3): mh-13-feed-detail.html wireframe, Public feed page (grid of cover photos), Listing detail page

### Community 99 - "README.md — Project Overview and Setup"
Cohesion: 0.67
Nodes (3): README.md — Project Overview and Setup, Conventional Commits convention, Trunk-based development branching strategy

### Community 100 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 101 - "logging.rs"
Cohesion: 0.16
Nodes (15): generates_a_valid_uuid_when_header_is_absent(), HEALTH_PATHS, propagates_incoming_x_request_id_header_verbatim(), request_id(), REQUEST_ID_HEADER, resolve_request_id(), HeaderMap, Next (+7 more)

### Community 107 - "RateLimitState"
Cohesion: 0.27
Nodes (10): middleware_passes_then_rejects_with_429_and_retry_after(), peer(), RateLimitState, Arc, AtomicU32, IpAddr, IpNet, Self (+2 more)

### Community 108 - "labels.ts"
Cohesion: 0.14
Nodes (19): 9. Critères de fin, 2.4 MH-57-FE : tests unitaires (vitest, logique pure), ListingStatus, ListListingsResult, updateListingStatus(), CHECKED_CLASSES, ListingStatusControlProps, STATUSES (+11 more)

### Community 109 - "OwnerListings.tsx"
Cohesion: 0.13
Nodes (16): 2.1 MH-55-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-55-BE — Tests unitaires (logique pure, sans DB), 2.3 MH-55-FE — Fichiers, dans l'ordre (un fichier à la fois), 2.4 MH-55-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, OwnerListings, PaginationMeta, OwnerListings() (+8 more)

### Community 110 - "RootLayout.tsx"
Cohesion: 0.17
Nodes (19): Décisions tranchées, 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE), 8. Hors périmètre, MH-55 — Les biens du propriétaire (« My properties ») : plan d'implémentation (BE + FE) (+11 more)

### Community 111 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 112 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 158 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 165 - "vite.config.ts"
Cohesion: 0.22
Nodes (7): MEDIA_CONTENT_TYPES, PUBLIC_MEDIA_PREFIXES, ref_node_fs, ref_node_path, @tailwindcss/vite, vite, @vitejs/plugin-react

### Community 166 - "file_validation.rs"
Cohesion: 0.11
Nodes (17): accepts_pdf(), accepts_supported_image_formats(), ALLOWED_IMAGE_TYPES, JPEG, MAX_IMAGE_SIZE_BYTES, MAX_PDF_SIZE_BYTES, MULTIPART_OVERHEAD_BYTES, PDF (+9 more)

### Community 167 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 168 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 169 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 174 - "overrides"
Cohesion: 0.67
Nodes (3): typescript, overrides, openapi-typescript

### Community 185 - "OwnerListingRow.tsx"
Cohesion: 0.11
Nodes (18): 1. Contexte et objectif, Décisions de conception prises pendant l'analyse, Décisions tranchées, ListingDetail(), ListingStatusControl(), OwnerBarProps, VISIBILITY_MESSAGE, OwnerListingRow() (+10 more)

### Community 186 - "vitest"
Cohesion: 0.18
Nodes (11): OwnerRequestStatus, STATUS_BADGE, decodeBase64Url(), readTokenRole(), Role, ROLES, base64Url(), tokenWithPayload() (+3 more)

### Community 187 - "OwnerRequestDetail.tsx"
Cohesion: 0.11
Nodes (19): OwnerRequestDetail, DocumentViewer(), OwnerRequestDetail(), handleApprove(), handleRejectConfirm(), OwnerRequestQueueList(), ownerRequestStatusBadge(), reviewErrorMessage() (+11 more)

### Community 188 - "super"
Cohesion: 0.33
Nodes (6): generate_otp_code(), generated_code_is_always_six_digits(), String, osrng, rng, super

### Community 190 - "2.3 MH-54-FE — Fichiers, dans l'ordre (un fichier à la fois)"
Cohesion: 0.40
Nodes (5): 2.1 MH-54-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-54-BE — Tests unitaires (logique pure, sans DB), 2.3 MH-54-FE — Fichiers, dans l'ordre (un fichier à la fois), 2.4 MH-54-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances

### Community 192 - "main.rs"
Cohesion: 0.33
Nodes (5): arc, backend_my_house, duration, socketaddr, tracing_subscriber

### Community 193 - "rate_limit"
Cohesion: 0.33
Nodes (6): rate_limit(), Next, Request, Response, State, ConnectInfo

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `Modern Villa Night Exterior with Pool (OIP 10)` → `Modern Villa Night Exterior with Pool (OIP 10)`  [AMBIGUOUS]
  frontend/docs-frontend/photo_my_house/OIP (10).webp · relation: conceptually_related_to
- `Embedded React/TypeScript Rules Content` → `README Writing Rules Skill`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with
- `react-typecrypt.md Rules File` → `README Writing Rules Skill`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with

## Knowledge Gaps
- **454 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+449 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 828 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **70 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `Modern Villa Night Exterior with Pool (OIP 10)` and `Modern Villa Night Exterior with Pool (OIP 10)`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Embedded React/TypeScript Rules Content` and `README Writing Rules Skill`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **What is the exact relationship between `react-typecrypt.md Rules File` and `README Writing Rules Skill`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **Why does `AppError` connect `AppError` to `listings/service.rs`, `owner_requests/service.rs`, `AppCacheProvider`, `validate_listing`, `Mailer`, `auth/repository.rs`, `jwt.rs`, `local_fs.rs`, `users/repository.rs`, `extractors.rs`, `validation.rs`, `users/service.rs`, `owner_requests/repository.rs`, `media/handler.rs`, `auth/handler.rs`, `media/service.rs`, `admin/handler.rs`, `file_validation.rs`, `users/handler.rs`, `owner_requests/handler.rs`, `media/repository.rs`, `errors.rs`, `rbac.rs`, `find_document_entry`, `listings/handler.rs`, `listings/repository.rs`, `ErrorBody`?**
  _High betweenness centrality (0.140) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState` to `listings/handler.rs`, `admin/handler.rs`, `AppCacheProvider`, `users/handler.rs`, `owner_requests/handler.rs`, `extractors.rs`, `app_server.rs`, `route.rs`, `health.rs`, `media/handler.rs`, `auth/handler.rs`?**
  _High betweenness centrality (0.030) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `config/mod.rs` to `Mailer`, `users/service.rs`, `AppState`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._