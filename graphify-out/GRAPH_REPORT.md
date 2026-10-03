# Graph Report - My-House  (2026-10-03)

## Corpus Check
- 332 files · ~175,278 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 24 file(s) not represented in the graph (top: (none) 15, .toml 5, .example 2)

## Summary
- 2510 nodes · 5604 edges · 188 communities (117 shown, 71 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 275 edges (avg confidence: 0.92)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `cd2eec8f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- listings/service.rs
- owner_requests/dto.rs
- auth/index.ts
- config/mod.rs
- owner_requests/service.rs
- storage_key.rs
- AppCacheProvider
- Alert
- components/index.ts
- Mailer
- AppError
- MyHouse Project Instructions (Agents)
- jwt.rs
- local_fs.rs
- users/repository.rs
- owner-request/api.ts
- AuthUser
- validation.rs
- EditListingPage.tsx
- users/dto.rs
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
- users/service.rs
- Design Handoff README
- Result
- What You Must Do When Invoked
- errors.rs
- users/handler.rs
- dependencies
- MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)
- admin identity-document read exception
- RecordingStorage
- owner_requests/handler.rs
- rate_limit.rs
- scripts
- ProfileForm.tsx
- client.ts
- media/repository.rs
- OwnerRequestForm.tsx
- CI Backend Workflow
- app_server.rs
- api/constants.ts
- OtpVerifyForm.tsx
- sqlx::query! / query_as! compile-time checked queries
- Badge.tsx
- users table
- mh-15-owner-request.html wireframe
- MH-53 — Créer un bien : plan d'implémentation (BE + FE)
- backend-dev service
- rbac.rs
- delete_storage_objects
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
- RootLayout.tsx
- owner_requests table
- tokenRole.ts
- react
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
- MH-58 — Supprimer un bien : plan d'implémentation (BE + FE)
- listings/api.ts
- RefreshTokenLookup
- MH-55 — Les biens du propriétaire (« My properties ») : plan d'implémentation (BE + FE)
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
- SiteHeader.tsx
- .claude/skills/graphify/references/extraction-spec.md
- .codex/skills/graphify/references/extraction-spec.md
- backend-my-house
- find_document_entry
- listings/index.ts
- pagination.ts
- super

## God Nodes (most connected - your core abstractions)
1. `AppError` - 195 edges
2. `AppState` - 69 edges
3. `cn()` - 46 edges
4. `react` - 42 edges
5. `AppCacheProvider` - 32 edges
6. `AuthUser` - 31 edges
7. `set_valid_env()` - 29 edges
8. `Alert()` - 28 edges
9. `ApiError` - 27 edges
10. `@tanstack/react-query` - 26 edges

## Surprising Connections (you probably didn't know these)
- `9. Critères de fin` --references--> `updateListingStatus()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/api.ts
- `9. Critères de fin` --references--> `deleteListing()`  [INFERRED]
  docs/plans/MH-58-plan.md → frontend/src/features/listings/api.ts
- `3.2 Fichiers, dans l'ordre (un fichier à la fois)` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-53_IMPLEMENTATION_PLAN.md → frontend/src/features/listings/components/ListingNotFound.tsx
- `Décisions prises` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/components/ListingNotFound.tsx
- `5. Vérifications` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-58-plan.md → frontend/src/features/listings/components/ListingNotFound.tsx

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

## Communities (188 total, 71 thin omitted)

### Community 0 - "listings/service.rs"
Cohesion: 0.05
Nodes (79): a_draft_summary_serializes_published_at_as_null(), a_published_summary_keeps_its_published_at(), ListingDetailDto, ListingMediaDto, ListingRequest, ListingStatusDto, ListingStatusRequest, ListingStatusResponse (+71 more)

### Community 1 - "owner_requests/dto.rs"
Cohesion: 0.12
Nodes (28): admin_detail_dto_falls_back_to_empty_documents_on_malformed_json(), admin_detail_dto_never_exposes_storage_key(), admin_detail_row(), admin_list_dto_omits_identity_data_and_documents(), admin_row(), AdminOwnerRequestDetailDto, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocumentDto (+20 more)

### Community 2 - "auth/index.ts"
Cohesion: 0.16
Nodes (20): OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken, RefreshResponse, refreshSession(), registerAccount(), RegisterPayload, RegisterResponse (+12 more)

### Community 3 - "config/mod.rs"
Cohesion: 0.07
Nodes (67): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, dev_allows_smtp_credentials_over_plaintext(), dev_without_smtp_credentials_sends_plaintext_unauthenticated(), empty_smtp_credentials_count_as_absent() (+59 more)

### Community 4 - "owner_requests/service.rs"
Cohesion: 0.10
Nodes (36): a_missing_storage_object_surfaces_as_document_not_found(), a_pdf_renamed_into_an_image_slot_is_a_shape_violation(), accepts_one_pdf_with_no_side(), accepts_two_images_as_front_and_back(), as_document_not_found(), as_shape_error(), ClassifiedDocument, classify_documents() (+28 more)

### Community 5 - "storage_key.rs"
Cohesion: 0.17
Nodes (16): assert_key_shape(), avatar_key(), avatar_key_from_url(), avatar_key_from_url_round_trips_a_generated_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key() (+8 more)

### Community 6 - "AppCacheProvider"
Cohesion: 0.05
Nodes (68): async_trait, AppCache, AUTH_CHALLENGE_MAX_ENTRIES, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache() (+60 more)

### Community 7 - "Alert"
Cohesion: 0.12
Nodes (20): 4. Points d'intégration avec l'existant, 2.3 MH-58-FE : fichiers, dans l'ordre (un fichier à la fois), 4. Points d'intégration avec l'existant, Décisions prises, OwnerActivationModalProps, DeleteListingActionProps, ListingNotFound(), REASONS (+12 more)

### Community 8 - "components/index.ts"
Cohesion: 0.07
Nodes (48): 2. Existant sur lequel on s'appuie, 2.3 MH-55-FE — Fichiers, dans l'ordre (un fichier à la fois), OwnerListings, OwnerListings(), parsePageParam(), AlertProps, AlertVariant, variantConfig (+40 more)

### Community 9 - "Mailer"
Cohesion: 0.05
Nodes (55): Address, AddressError, AsyncSmtpTransport, authenticates_with_configured_credentials(), builds_successfully_with_valid_config(), builds_with_credentials_and_each_security_mode(), credentials(), hello_name() (+47 more)

### Community 10 - "AppError"
Cohesion: 0.24
Nodes (20): create_account(), db_err(), email_exists(), find_by_hash(), find_user_by_email(), insert_refresh_token(), revoke(), revoke_all_for_user() (+12 more)

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

### Community 15 - "owner-request/api.ts"
Cohesion: 0.17
Nodes (17): getOwnerRequestStatus(), OwnerRequest, OwnerRequestResponse, ownerRequestStatusQueryKey, OwnerRequestStatusResponse, submitOwnerRequest(), OwnerRequestStatus(), useOwnerRequestStatus() (+9 more)

### Community 16 - "AuthUser"
Cohesion: 0.11
Nodes (29): AppJson<T>, AuthState, AuthUser, bearer_token(), MaybeAuthUser, optional_bearer_token(), resolve_identity(), resolve_identity_rejects_an_undecodable_token_without_touching_cache_or_db() (+21 more)

### Community 17 - "validation.rs"
Cohesion: 0.08
Nodes (24): FieldErrors, finish_reports_one_entry_per_recorded_violation(), MAX_ADMIN_NOTE_LENGTH, MAX_NAME_LENGTH, MAX_PHONE_LENGTH, normalize_place_name(), NUL, optional_name() (+16 more)

### Community 18 - "EditListingPage.tsx"
Cohesion: 0.13
Nodes (30): 4.2 Fichiers, dans l'ordre (un fichier à la fois), Décisions de conception prises pendant l'analyse (sans question), 1. Contexte et objectif, 2.1 MH-56-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-56-BE — Tests unitaires (logique pure, sans DB), 2.3 MH-56-FE — Fichiers, dans l'ordre (un fichier à la fois), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé (+22 more)

### Community 19 - "users/dto.rs"
Cohesion: 0.12
Nodes (18): AvatarUploadForm, response_envelope_serializes_the_profile_fields(), row(), row_maps_to_dto_field_for_field(), From, Option, Role, Self (+10 more)

### Community 20 - "admin/api.ts"
Cohesion: 0.16
Nodes (20): AdminOwnerRequest, AdminOwnerRequestDetail, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocument, getOwnerRequest(), getOwnerRequestDocument(), listOwnerRequests(), ListOwnerRequestsParams (+12 more)

### Community 21 - "AppState"
Cohesion: 0.10
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
Cohesion: 0.24
Nodes (15): delete_media(), FILE_FIELD_NAME, LISTING_ID_FIELD_NAME, multipart_error(), promote_cover(), read_submission(), Json, Multipart (+7 more)

### Community 29 - "compilerOptions"
Cohesion: 0.11
Nodes (17): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+9 more)

### Community 30 - "auth/handler.rs"
Cohesion: 0.18
Nodes (21): logout(), otp_request(), otp_verify(), refresh(), register(), CookieJar, Json, Result (+13 more)

### Community 31 - "20260729041038_baseline_schema.sql"
Cohesion: 0.13
Nodes (25): idx_listing_media_listing, idx_listing_media_one_cover, idx_listings_city, idx_listings_city_normalized, idx_listings_owner, idx_listings_search, idx_listings_status, idx_listings_type (+17 more)

### Community 32 - "ListingForm.tsx"
Cohesion: 0.10
Nodes (37): 4.1 Préalable, 4.3 Tests unitaires (vitest, logique pure), 4.4 Vérifications FE, 4. MH-53-FE — Formulaire « Publish a listing », 2.4 MH-56-FE — Tests unitaires (vitest, logique pure), 6. Risques et inconnues, ListingDetail, ListingForm() (+29 more)

### Community 33 - "media/service.rs"
Cohesion: 0.05
Nodes (62): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+54 more)

### Community 34 - "admin/handler.rs"
Cohesion: 0.26
Nodes (15): AdminOwnerRequestDetailResponse, get_owner_request(), get_owner_request_document(), list_owner_requests(), review_owner_request(), IntoResponse, Json, Path (+7 more)

### Community 35 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 36 - "users/service.rs"
Cohesion: 0.28
Nodes (16): bootstrap_admin(), delete_account(), delete_previous_avatar(), deletes_the_key_behind_the_previous_avatar_url(), get_me(), replace_avatar(), Bytes, Option (+8 more)

### Community 37 - "Design Handoff README"
Cohesion: 0.14
Nodes (16): MyHouse Design Tokens (Plan & Brass), Dimension Rule Signature Device, MH-17 Superseded Cream/Terracotta Palette, Design Handoff Repo Sync Log & Screen Map, My House - Plan & Laiton Design Mockup, My House - Screens Design Mockup, Design Handoff README, Admin Approve as Sole Solid Semantic Button (+8 more)

### Community 38 - "Result"
Cohesion: 0.24
Nodes (8): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage

### Community 39 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 40 - "errors.rs"
Cohesion: 0.06
Nodes (44): axum, check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus (+36 more)

### Community 41 - "users/handler.rs"
Cohesion: 0.22
Nodes (17): AVATAR_FIELD_NAME, delete_me(), get_me(), multipart_error(), read_file_field(), Bytes, CookieJar, Json (+9 more)

### Community 42 - "dependencies"
Cohesion: 0.25
Nodes (8): dependencies, clsx, lucide-react, react, react-dom, react-hook-form, react-router, @tanstack/react-query

### Community 43 - "MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)"
Cohesion: 0.14
Nodes (13): 1. Contexte et objectif, 2.1 MH-54-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-54-BE — Tests unitaires (logique pure, sans DB), 2.4 MH-54-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications, 6. Risques et inconnues (+5 more)

### Community 44 - "admin identity-document read exception"
Cohesion: 0.15
Nodes (14): admin identity-document read exception, ARCHITECTURE.md (referenced doc), backend/Dockerfile, docker-compose.yml, backend/.env.example variables, frontend/Dockerfile, GET /admin/owner-requests/:id/documents/:doc_id, infra/mailer.rs (lettre SMTP client) (+6 more)

### Community 45 - "RecordingStorage"
Cohesion: 0.23
Nodes (9): RecordingStorage, Bytes, Duration, Mutex, Result, Self, StorageProvider, String (+1 more)

### Community 46 - "owner_requests/handler.rs"
Cohesion: 0.15
Nodes (18): DOCUMENTS_FIELD_NAME, get_owner_request_status(), multipart_error(), read_submission(), Json, Multipart, MultipartError, Result (+10 more)

### Community 47 - "rate_limit.rs"
Cohesion: 0.06
Nodes (56): apidoc, appenv, apperror, arc, atomic, backend_my_house, ApiDoc, allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it() (+48 more)

### Community 48 - "scripts"
Cohesion: 0.15
Nodes (13): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+5 more)

### Community 49 - "ProfileForm.tsx"
Cohesion: 0.19
Nodes (16): ProfileForm, RegistrationFormProps, ProfileFields(), handleSubmit(), ProfileForm(), ROLE_LABELS, FIELDS_BY_SERVER_NAME, maxLengthMessage() (+8 more)

### Community 50 - "client.ts"
Cohesion: 0.12
Nodes (24): 2.1 MH-58-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-58-BE : tests unitaires (logique pure, sans DB), 2.4 MH-58-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, isAlreadyDeleted(), AccessTokenGetter, ApiError (+16 more)

### Community 51 - "media/repository.rs"
Cohesion: 0.36
Nodes (14): count_media(), db_err(), delete_media(), demote_other_covers(), find_listing_media(), insert_media(), lock_listing_owner(), lock_media_for_deletion() (+6 more)

### Community 52 - "OwnerRequestForm.tsx"
Cohesion: 0.12
Nodes (12): OwnerRequestForm, FileWithPreview, FormValues, ID_TYPE_OPTIONS, ImageSlotProps, Modality, PdfSlotProps, FileDropzone() (+4 more)

### Community 53 - "CI Backend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 54 - "app_server.rs"
Cohesion: 0.18
Nodes (11): AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), Box, build_router (+3 more)

### Community 55 - "api/constants.ts"
Cohesion: 0.12
Nodes (24): OwnerRequestForm(), GENERIC_ERROR_MESSAGE, IMAGE_FORMAT_MESSAGE, IMAGE_SIZE_MESSAGE, PDF_FORMAT_MESSAGE, PDF_SIZE_MESSAGE, preCheckImage(), preCheckPdf() (+16 more)

### Community 56 - "OtpVerifyForm.tsx"
Cohesion: 0.11
Nodes (23): AuthFlow, requestOtp(), AuthFlow(), leaveRegistration(), AuthStepIndicator(), markInterrupted(), readInterrupted(), Screen (+15 more)

### Community 57 - "sqlx::query! / query_as! compile-time checked queries"
Cohesion: 0.22
Nodes (10): DATABASE_URL compile-time requirement, fn_set_updated_at trigger, Partial unique indexes for 'at most one active X' invariants, sqlx::query! / query_as! compile-time checked queries, sqlx QueryBuilder, Repository tests in rolled-back transaction (integration feature flag), Schema conventions (UUID PK, timestamps, enums), No string-interpolated SQL rule (+2 more)

### Community 58 - "Badge.tsx"
Cohesion: 0.14
Nodes (13): OwnerRequestDetail(), handleApprove(), handleRejectConfirm(), OwnerRequestQueueList(), OwnerRequestStatus, ownerRequestStatusBadge(), STATUS_BADGE, reviewErrorMessage() (+5 more)

### Community 59 - "users table"
Cohesion: 0.24
Nodes (10): fn_cascade_owner_name_to_listings trigger fn, fn_update_listing_search_vector trigger fn, listing_status enum, listing_type enum, listings module, listings table, search module, user_role enum (+2 more)

### Community 60 - "mh-15-owner-request.html wireframe"
Cohesion: 0.20
Nodes (10): mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire'), Identity document dropzone component, Request approved status page (+2 more)

### Community 61 - "MH-53 — Créer un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.18
Nodes (10): 3.1 Contrat, 3.2 Fichiers, dans l'ordre (un fichier à la fois), 3.3 Tests unitaires (logique pure, sans DB), 3.4 Vérifications BE, 3. MH-53-BE — Création d'un bien, 5. Déroulé imposé pour chaque sous-ticket, 6. Hors périmètre, 7. Critères de fin (+2 more)

### Community 62 - "backend-dev service"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 63 - "rbac.rs"
Cohesion: 0.24
Nodes (4): failed_check_maps_to_forbidden(), require_role(), Role, Result

### Community 64 - "delete_storage_objects"
Cohesion: 0.39
Nodes (7): a_missing_or_failing_key_does_not_abort_the_remaining_keys(), delete_storage_objects(), deletes_every_enumerated_key(), no_keys_is_a_no_op(), StorageProvider, String, recordingstorage

### Community 65 - "listings/handler.rs"
Cohesion: 0.21
Nodes (22): ListingDetailResponse, create(), delete(), get_by_id(), list(), list_mine(), OWNER_ROLES, Json (+14 more)

### Community 66 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 67 - "Toast.tsx"
Cohesion: 0.14
Nodes (14): App(), Providers(), ProvidersProps, queryClient, router, frontend_src_index, ToastContext, ToastContextValue (+6 more)

### Community 68 - "listing_media table"
Cohesion: 0.25
Nodes (8): automatic cover photo selection, AwsS3Storage (V2), listing_media table, LocalFsStorage, magic-bytes file validation, Media API endpoints (§4.5), server-generated (UUID) filenames, StorageProvider trait

### Community 69 - "package.json"
Cohesion: 0.11
Nodes (19): name, private, type, version, clsx, eslint, @eslint/js, eslint-plugin-react-hooks (+11 more)

### Community 70 - "listings/repository.rs"
Cohesion: 0.20
Nodes (27): count_listings(), count_owner_listings(), delete_listing(), find_listing_by_id(), find_media_for_listing(), insert_listing(), list_listings(), list_media_keys() (+19 more)

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
Cohesion: 0.25
Nodes (12): deleteAccount(), getMe(), profileQueryKey, updateMe(), UpdateProfilePayload, uploadAvatar(), UserResponse, useDeleteAccount() (+4 more)

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
Nodes (17): 1. Décisions tranchées, 1. Contexte et objectif, 2.1 MH-57-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-57-BE : tests unitaires (logique pure, sans DB), 2.3 MH-57-FE : fichiers, dans l'ordre (un fichier à la fois), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications (+9 more)

### Community 83 - "db.rs"
Cohesion: 0.29
Nodes (7): connect_db(), MAX_CONNECTIONS, Error, PgPool, Result, sqlx::PgPool connection pooling, pgpooloptions

### Community 84 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 85 - "OtpCodeInput"
Cohesion: 0.57
Nodes (7): OtpCodeInput(), focusInput(), handleChange(), handleKeyDown(), handlePaste(), setDigit(), submit()

### Community 86 - "RootLayout.tsx"
Cohesion: 0.36
Nodes (12): 2.3 MH-54-FE — Fichiers, dans l'ordre (un fichier à la fois), Décisions tranchées, 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant, AdminLayout(), handleSignOut(), RootLayout(), signOutTo() (+4 more)

### Community 87 - "owner_requests table"
Cohesion: 0.40
Nodes (5): owner request atomic submission, owner_request_status enum, owner_requests table, PATCH /admin/owner-requests/:id, POST /owner-requests

### Community 88 - "tokenRole.ts"
Cohesion: 0.27
Nodes (8): decodeBase64Url(), readTokenRole(), Role, ROLES, base64Url(), tokenWithPayload(), frontend_src_shared_api_types, frontend_src_shared_api_types_components

### Community 89 - "react"
Cohesion: 0.13
Nodes (21): AuthLayout(), RequireAdminProps, RequireAuth(), RequireAuthProps, RequireOwnerProps, CreateListingPage, EditListingPage, ListingDetail (+13 more)

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
Cohesion: 0.15
Nodes (16): generates_a_valid_uuid_when_header_is_absent(), HEALTH_PATHS, propagates_incoming_x_request_id_header_verbatim(), request_id(), REQUEST_ID_HEADER, resolve_request_id(), HeaderMap, Next (+8 more)

### Community 107 - "MH-58 — Supprimer un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.22
Nodes (8): 1. Contexte et objectif, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (sur chaque branche, BE puis FE), 8. Hors périmètre, 9. Critères de fin, MH-58 — Supprimer un bien : plan d'implémentation (BE + FE), Outils vérifiés pendant l'analyse

### Community 108 - "listings/api.ts"
Cohesion: 0.17
Nodes (21): 9. Critères de fin, 2.4 MH-57-FE : tests unitaires (vitest, logique pure), deleteListing(), ListingStatus, ListListingsResult, updateListingStatus(), CHECKED_CLASSES, ListingStatusControl() (+13 more)

### Community 109 - "RefreshTokenLookup"
Cohesion: 0.47
Nodes (5): NewAccount, RefreshTokenLookup, Option, Role, Uuid

### Community 110 - "MH-55 — Les biens du propriétaire (« My properties ») : plan d'implémentation (BE + FE)"
Cohesion: 0.14
Nodes (13): 1. Contexte et objectif, 2.1 MH-55-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-55-BE — Tests unitaires (logique pure, sans DB), 2.4 MH-55-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE) (+5 more)

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

### Community 175 - "SiteHeader.tsx"
Cohesion: 0.12
Nodes (11): MobileMenu(), NavItem(), NavLinkSpec, OwnerHeader(), ownerLinks(), PUBLIC_LINKS, SeekerHeader(), SiteHeaderProps (+3 more)

### Community 184 - "find_document_entry"
Cohesion: 0.60
Nodes (5): a_doc_id_from_a_different_request_does_not_resolve(), documents_json(), find_document_entry(), finds_the_document_matching_doc_id_in_its_own_request(), Value

### Community 185 - "listings/index.ts"
Cohesion: 0.10
Nodes (31): Décisions de conception prises pendant l'analyse, 4. Points d'intégration avec l'existant, getListing(), ListingSummary, ListingType, listListings(), ListListingsParams, listOwnerListings() (+23 more)

### Community 188 - "super"
Cohesion: 0.33
Nodes (6): generate_otp_code(), generated_code_is_always_six_digits(), String, osrng, rng, super

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
- **457 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+452 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 836 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **71 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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
- **Why does `AppError` connect `AppError` to `listings/service.rs`, `owner_requests/service.rs`, `AppCacheProvider`, `Mailer`, `jwt.rs`, `local_fs.rs`, `users/repository.rs`, `AuthUser`, `validation.rs`, `owner_requests/repository.rs`, `media/handler.rs`, `auth/handler.rs`, `media/service.rs`, `admin/handler.rs`, `users/service.rs`, `Result`, `file_validation.rs`, `errors.rs`, `users/handler.rs`, `RecordingStorage`, `owner_requests/handler.rs`, `media/repository.rs`, `find_document_entry`, `rbac.rs`, `listings/handler.rs`, `listings/repository.rs`?**
  _High betweenness centrality (0.364) - this node is a cross-community bridge._
- **Why does `upload()` connect `media/service.rs` to `AppError`, `storage_key.rs`, `file_validation.rs`, `Alert`?**
  _High betweenness centrality (0.296) - this node is a cross-community bridge._
- **Why does `4. Points d'intégration avec l'existant` connect `Alert` to `media/service.rs`, `MH-58 — Supprimer un bien : plan d'implémentation (BE + FE)`, `listings/api.ts`, `RootLayout.tsx`?**
  _High betweenness centrality (0.296) - this node is a cross-community bridge._