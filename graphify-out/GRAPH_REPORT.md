# Graph Report - My-House  (2026-10-28)

## Corpus Check
- 377 files · ~214,328 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 24 file(s) not represented in the graph (top: (none) 15, .toml 5, .example 2)

## Summary
- 2890 nodes · 6853 edges · 208 communities (141 shown, 67 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 553 edges (avg confidence: 0.93)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8ccc5414`
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
- MediaRow
- components/index.ts
- Mailer
- auth/repository.rs
- MyHouse Project Instructions (Agents)
- jwt.rs
- local_fs.rs
- users/repository.rs
- Alert
- extractors.rs
- validation.rs
- listings/api.ts
- OwnerRequestForm.tsx
- admin/api.ts
- AppState
- compilerOptions
- Code Review — Backend Skill
- owner_requests/repository.rs
- devDependencies
- Module: users
- owner_requests/model.rs
- MH-53 — Créer un bien : plan d'implémentation (BE + FE)
- compilerOptions
- auth/handler.rs
- 20260729041038_baseline_schema.sql
- PhotoTile.tsx
- media/service.rs
- AuthUser
- .mcp.json
- users/service.rs
- Design Handoff README
- Result
- /graphify
- pagination.rs
- users/handler.rs
- dependencies
- MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)
- admin identity-document read exception
- RecordingStorage
- owner_requests/handler.rs
- rate_limit.rs
- scripts
- profile/api.ts
- users/dto.rs
- media/repository.rs
- MH-57 — Changer le statut d'un bien : plan d'implémentation (BE + FE)
- CI Backend Workflow
- app_server.rs
- auth/dto.rs
- OtpVerifyForm.tsx
- sqlx::query! / query_as! compile-time checked queries
- 2. Découpage BE / FE et dépendances
- users table
- mh-15-owner-request.html wireframe
- Result
- backend-dev service
- OwnerBar
- delete_storage_objects
- AppError
- Docker Rules Skill
- health.rs
- listing_media table
- package.json
- listings/repository.rs
- CI Job: Frontend OpenAPI Codegen
- CLAUDE.md — MyHouse project instructions
- ON DELETE CASCADE — relational-only cleanup rule
- avatarValidation.ts
- graphify reference: extra exports and benchmark
- users module
- shared module
- Auth API endpoints (§4.1)
- Owner Request Rejected Email Template
- Embedded React/TypeScript Rules Content
- graphify reference: extra exports and benchmark
- MH-59 — Écran de gestion des photos d'un bien : plan d'implémentation (FE)
- db.rs
- /graphify
- useListingPhotoMutations.ts
- route.rs
- owner_requests table
- client.ts
- ListingPhotosPage.tsx
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
- ListingForm.tsx
- AuthUser Extractor
- No-Proxy Principle for Public Files
- R-08: search_vector trigger N+1 query
- Refresh Token httpOnly Cookie
- GET /users/me/owner-request
- RateLimitState
- @tanstack/react-query
- auth/api.ts
- listings/index.ts
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- pre-commit
- media_order.rs
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
- searchCriteria.ts
- MH-66 — Libellés de type anglais dans l'index de recherche : plan d'implémentation (BE)
- vitest
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
- listingPhotos.ts
- .claude/skills/graphify/references/extraction-spec.md
- .codex/skills/graphify/references/extraction-spec.md
- backend-my-house
- MH-60 — Réordonner les photos d'un bien : plan d'implémentation (BE + FE)
- cors.rs
- media/handler.rs
- rbac.rs
- errors.rs
- ListingPhotosPage
- logging.rs
- What You Must Do When Invoked
- What You Must Do When Invoked
- Toast.tsx
- owner-request/api.ts
- Step 3 - Extract entities and relationships
- Step 3 - Extract entities and relationships
- UserMenu
- image
- react
- ApiError
- OwnerRequestDetail
- AuthContext.tsx
- main.rs
- rate_limit
- ErrorBody
- ListingForm
- super

## God Nodes (most connected - your core abstractions)
1. `AppError` - 213 edges
2. `AppState` - 72 edges
3. `cn()` - 54 edges
4. `react` - 51 edges
5. `Alert()` - 43 edges
6. `ApiError` - 37 edges
7. `AppCacheProvider` - 32 edges
8. `AuthUser` - 32 edges
9. `@tanstack/react-query` - 30 edges
10. `set_valid_env()` - 29 edges

## Surprising Connections (you probably didn't know these)
- `9. Critères de fin` --references--> `updateListingStatus()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/api.ts
- `3.2 Fichiers, dans l'ordre (un fichier à la fois)` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-53_IMPLEMENTATION_PLAN.md → frontend/src/features/listings/components/ListingNotFound.tsx
- `Décisions prises` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/components/ListingNotFound.tsx
- `5. Vérifications` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-58-plan.md → frontend/src/features/listings/components/ListingNotFound.tsx
- `2.1 MH-61-BE : fichiers, dans l'ordre (un fichier à la fois)` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-61-plan.md → frontend/src/features/listings/components/ListingNotFound.tsx

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

## Communities (208 total, 67 thin omitted)

### Community 0 - "listings/service.rs"
Cohesion: 0.06
Nodes (70): a_draft_summary_serializes_published_at_as_null(), a_published_summary_keeps_its_published_at(), ListingDetailDto, ListingMediaDto, ListingRequest, ListingStatusDto, ListingStatusRequest, ListingStatusResponse (+62 more)

### Community 1 - "owner_requests/dto.rs"
Cohesion: 0.12
Nodes (28): admin_detail_dto_falls_back_to_empty_documents_on_malformed_json(), admin_detail_dto_never_exposes_storage_key(), admin_detail_row(), admin_list_dto_omits_identity_data_and_documents(), admin_row(), AdminOwnerRequestDetailDto, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocumentDto (+20 more)

### Community 2 - "auth/index.ts"
Cohesion: 0.14
Nodes (16): AuthFlow, registerAccount(), AuthContextValue, AuthStatus, AuthFlow(), leaveRegistration(), AuthStepIndicator(), markInterrupted() (+8 more)

### Community 3 - "config/mod.rs"
Cohesion: 0.07
Nodes (64): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, dev_allows_smtp_credentials_over_plaintext(), dev_without_smtp_credentials_sends_plaintext_unauthenticated(), empty_smtp_credentials_count_as_absent() (+56 more)

### Community 4 - "owner_requests/service.rs"
Cohesion: 0.09
Nodes (41): a_doc_id_from_a_different_request_does_not_resolve(), a_missing_storage_object_surfaces_as_document_not_found(), a_pdf_renamed_into_an_image_slot_is_a_shape_violation(), accepts_one_pdf_with_no_side(), accepts_two_images_as_front_and_back(), as_document_not_found(), as_shape_error(), ClassifiedDocument (+33 more)

### Community 5 - "storage_key.rs"
Cohesion: 0.17
Nodes (16): assert_key_shape(), avatar_key(), avatar_key_from_url(), avatar_key_from_url_round_trips_a_generated_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key() (+8 more)

### Community 6 - "AppCacheProvider"
Cohesion: 0.05
Nodes (68): async_trait, AppCache, AUTH_CHALLENGE_MAX_ENTRIES, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache() (+60 more)

### Community 7 - "MediaRow"
Cohesion: 0.14
Nodes (21): MediaDto, MediaListResponse, MediaResponse, PromoteCoverRequest, ReorderMediaRequest, From, Option, Self (+13 more)

### Community 8 - "components/index.ts"
Cohesion: 0.05
Nodes (59): 2.3 MH-64-FE : fichiers, dans l'ordre (un fichier à la fois), AlertProps, AlertVariant, variantConfig, BadgeProps, BadgeSize, sizeClasses, toneClasses (+51 more)

### Community 9 - "Mailer"
Cohesion: 0.05
Nodes (55): Address, AddressError, AsyncSmtpTransport, authenticates_with_configured_credentials(), builds_successfully_with_valid_config(), builds_with_credentials_and_each_security_mode(), credentials(), hello_name() (+47 more)

### Community 10 - "auth/repository.rs"
Cohesion: 0.21
Nodes (20): NewAccount, RefreshTokenLookup, Option, Role, Uuid, create_account(), db_err(), email_exists() (+12 more)

### Community 11 - "MyHouse Project Instructions (Agents)"
Cohesion: 0.06
Nodes (36): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+28 more)

### Community 12 - "jwt.rs"
Cohesion: 0.10
Nodes (36): Algorithm, hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest(), String, sha256_hex(), Claims, encode_with_exp() (+28 more)

### Community 13 - "local_fs.rs"
Cohesion: 0.15
Nodes (25): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, OWNER_REQUESTS_PREFIX, presigned_url_returns_not_implemented_error(), read_on_missing_key_returns_not_found_not_generic_storage_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes() (+17 more)

### Community 14 - "users/repository.rs"
Cohesion: 0.37
Nodes (15): admin_exists(), delete_by_id(), find_by_id(), find_is_active(), list_listing_media_keys_for_owner(), list_owner_request_document_keys(), Option, PgPool (+7 more)

### Community 15 - "Alert"
Cohesion: 0.07
Nodes (41): 1. Contexte et objectif, Décisions de conception prises pendant l'analyse, Décisions tranchées, 4. Points d'intégration avec l'existant, 1. Contexte et objectif, 2.1 Fichiers, dans l'ordre (un fichier à la fois), 2.2 Ce qui ne change pas, 2.3 Vérification (+33 more)

### Community 16 - "extractors.rs"
Cohesion: 0.10
Nodes (30): AppJson<T>, AppQuery<T>, AuthState, bearer_token(), MaybeAuthUser, optional_bearer_token(), resolve_identity(), resolve_identity_rejects_an_undecodable_token_without_touching_cache_or_db() (+22 more)

### Community 17 - "validation.rs"
Cohesion: 0.07
Nodes (26): FieldErrors, finish_reports_one_entry_per_recorded_violation(), keyword_filter_counts_characters_not_bytes(), MAX_ADMIN_NOTE_LENGTH, MAX_KEYWORD_LENGTH, MAX_NAME_LENGTH, MAX_PHONE_LENGTH, normalize_place_name() (+18 more)

### Community 18 - "listings/api.ts"
Cohesion: 0.07
Nodes (50): 2. Existant sur lequel on s'appuie, 2.1 MH-63-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-63-BE : tests unitaires (logique pure, sans DB), 2.3 MH-63-FE : fichiers, dans l'ordre (un fichier à la fois), 2.4 MH-63-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant (+42 more)

### Community 19 - "OwnerRequestForm.tsx"
Cohesion: 0.09
Nodes (31): OwnerRequestForm, FileWithPreview, FormValues, ID_TYPE_OPTIONS, ImageSlotProps, Modality, OwnerRequestForm(), PdfSlotProps (+23 more)

### Community 20 - "admin/api.ts"
Cohesion: 0.21
Nodes (16): AdminOwnerRequest, AdminOwnerRequestDetail, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocument, getOwnerRequest(), getOwnerRequestDocument(), listOwnerRequests(), ListOwnerRequestsParams (+8 more)

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

### Community 28 - "MH-53 — Créer un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.18
Nodes (10): 3.1 Contrat, 3.2 Fichiers, dans l'ordre (un fichier à la fois), 3.3 Tests unitaires (logique pure, sans DB), 3.4 Vérifications BE, 3. MH-53-BE — Création d'un bien, 5. Déroulé imposé pour chaque sous-ticket, 6. Hors périmètre, 7. Critères de fin (+2 more)

### Community 29 - "compilerOptions"
Cohesion: 0.11
Nodes (17): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+9 more)

### Community 30 - "auth/handler.rs"
Cohesion: 0.21
Nodes (19): logout(), otp_request(), otp_verify(), refresh(), register(), CookieJar, Json, Result (+11 more)

### Community 31 - "20260729041038_baseline_schema.sql"
Cohesion: 0.11
Nodes (27): idx_listing_media_listing, idx_listing_media_one_cover, idx_listings_city, idx_listings_city_normalized, idx_listings_owner, idx_listings_search, idx_listings_status, idx_listings_type (+19 more)

### Community 32 - "PhotoTile.tsx"
Cohesion: 0.14
Nodes (11): 6. Risques et inconnues, ListingMedia, MoveButtonProps, PendingPhotoTile(), PendingPhotoTileProps, PhotoTile(), PhotoTileProps, TileFrameProps (+3 more)

### Community 33 - "media/service.rs"
Cohesion: 0.07
Nodes (19): a_duplicated_id_is_a_conflict(), a_missing_id_is_a_conflict(), an_extra_id_is_a_conflict(), an_unknown_id_of_the_same_length_is_a_conflict(), another_owners_photo_is_reported_as_not_found(), db_err(), exact_match_stores_the_sent_order_with_the_cover_first(), first_photo_becomes_the_cover_at_position_zero() (+11 more)

### Community 34 - "AuthUser"
Cohesion: 0.22
Nodes (17): AdminOwnerRequestDetailResponse, get_owner_request(), get_owner_request_document(), list_owner_requests(), review_owner_request(), IntoResponse, Json, Path (+9 more)

### Community 35 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 36 - "users/service.rs"
Cohesion: 0.22
Nodes (19): UpdateMeDto, bootstrap_admin(), delete_account(), delete_previous_avatar(), deletes_the_key_behind_the_previous_avatar_url(), get_me(), replace_avatar(), Bytes (+11 more)

### Community 37 - "Design Handoff README"
Cohesion: 0.14
Nodes (16): MyHouse Design Tokens (Plan & Brass), Dimension Rule Signature Device, MH-17 Superseded Cream/Terracotta Palette, Design Handoff Repo Sync Log & Screen Map, My House - Plan & Laiton Design Mockup, My House - Screens Design Mockup, Design Handoff README, Admin Approve as Sole Solid Semantic Button (+8 more)

### Community 38 - "Result"
Cohesion: 0.24
Nodes (8): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage

### Community 39 - "/graphify"
Cohesion: 0.20
Nodes (9): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Usage (+1 more)

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
Cohesion: 0.13
Nodes (22): apperror, atomic, allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it(), distinct_clients_get_distinct_counters(), falls_back_to_peer_ip_when_trusted_header_is_missing(), headers_with_xff(), ignores_x_forwarded_for_from_an_untrusted_peer(), NGINX (+14 more)

### Community 48 - "scripts"
Cohesion: 0.15
Nodes (13): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+5 more)

### Community 49 - "profile/api.ts"
Cohesion: 0.29
Nodes (11): deleteAccount(), getMe(), profileQueryKey, updateMe(), UpdateProfilePayload, uploadAvatar(), UserResponse, useUpdateProfile() (+3 more)

### Community 50 - "users/dto.rs"
Cohesion: 0.12
Nodes (17): AvatarUploadForm, response_envelope_serializes_the_profile_fields(), row(), row_maps_to_dto_field_for_field(), From, Option, Role, Self (+9 more)

### Community 51 - "media/repository.rs"
Cohesion: 0.30
Nodes (18): count_media(), db_err(), delete_media(), demote_other_covers(), find_listing_media(), insert_media(), list_listing_media(), lock_listing_owner() (+10 more)

### Community 52 - "MH-57 — Changer le statut d'un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.14
Nodes (14): 1. Décisions tranchées, 1. Contexte et objectif, 2.1 MH-57-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-57-BE : tests unitaires (logique pure, sans DB), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications, 6. Risques et inconnues (+6 more)

### Community 53 - "CI Backend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 54 - "app_server.rs"
Cohesion: 0.18
Nodes (11): AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), Box, build_router (+3 more)

### Community 55 - "auth/dto.rs"
Cohesion: 0.26
Nodes (13): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+5 more)

### Community 56 - "OtpVerifyForm.tsx"
Cohesion: 0.12
Nodes (24): requestOtp(), verifyOtp(), OtpCodeInput(), focusInput(), handleChange(), handleKeyDown(), handlePaste(), setDigit() (+16 more)

### Community 57 - "sqlx::query! / query_as! compile-time checked queries"
Cohesion: 0.22
Nodes (10): DATABASE_URL compile-time requirement, fn_set_updated_at trigger, Partial unique indexes for 'at most one active X' invariants, sqlx::query! / query_as! compile-time checked queries, sqlx QueryBuilder, Repository tests in rolled-back transaction (integration feature flag), Schema conventions (UUID PK, timestamps, enums), No string-interpolated SQL rule (+2 more)

### Community 58 - "2. Découpage BE / FE et dépendances"
Cohesion: 0.50
Nodes (4): 2.1 MH-58-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-58-BE : tests unitaires (logique pure, sans DB), 2.4 MH-58-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances

### Community 59 - "users table"
Cohesion: 0.24
Nodes (10): fn_cascade_owner_name_to_listings trigger fn, fn_update_listing_search_vector trigger fn, listing_status enum, listing_type enum, listings module, listings table, search module, user_role enum (+2 more)

### Community 60 - "mh-15-owner-request.html wireframe"
Cohesion: 0.20
Nodes (10): mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire'), Identity document dropzone component, Request approved status page (+2 more)

### Community 61 - "Result"
Cohesion: 0.23
Nodes (22): apply_order(), check_upload_slot(), cover_promotion(), delete(), delete_decision(), persist_media(), promote_cover(), reorder_media() (+14 more)

### Community 62 - "backend-dev service"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 63 - "OwnerBar"
Cohesion: 0.18
Nodes (17): 1. Contexte et objectif, 2.1 MH-56-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-56-BE — Tests unitaires (logique pure, sans DB), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications, 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE), 8. Hors périmètre (+9 more)

### Community 64 - "delete_storage_objects"
Cohesion: 0.39
Nodes (7): a_missing_or_failing_key_does_not_abort_the_remaining_keys(), delete_storage_objects(), deletes_every_enumerated_key(), no_keys_is_a_no_op(), StorageProvider, String, recordingstorage

### Community 65 - "AppError"
Cohesion: 0.15
Nodes (35): ListingDetailResponse, create(), delete(), get_by_id(), list(), list_mine(), OWNER_ROLES, publish() (+27 more)

### Community 66 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 67 - "health.rs"
Cohesion: 0.21
Nodes (10): axum, check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus (+2 more)

### Community 68 - "listing_media table"
Cohesion: 0.25
Nodes (8): automatic cover photo selection, AwsS3Storage (V2), listing_media table, LocalFsStorage, magic-bytes file validation, Media API endpoints (§4.5), server-generated (UUID) filenames, StorageProvider trait

### Community 69 - "package.json"
Cohesion: 0.11
Nodes (19): name, private, type, version, clsx, eslint, @eslint/js, eslint-plugin-react-hooks (+11 more)

### Community 70 - "listings/repository.rs"
Cohesion: 0.06
Nodes (63): Self, ListingDetailRow, ListingFields, ListingMediaRow, ListingStatus, ListingSummaryRow, ListingType, Option (+55 more)

### Community 71 - "CI Job: Frontend OpenAPI Codegen"
Cohesion: 0.43
Nodes (8): CI Job: Frontend Build, CI Job: Frontend OpenAPI Codegen, CI Job: Frontend Docker Build, CI Job: Frontend Install, CI Job: Frontend Lint, CI Job: Frontend Prettier, CI Job: Frontend Unit Tests, CI Job: Frontend TypeScript

### Community 72 - "CLAUDE.md — MyHouse project instructions"
Cohesion: 0.29
Nodes (7): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), mcp/.toolbox/tool.yaml — postgres-local MCP toolbox source

### Community 73 - "ON DELETE CASCADE — relational-only cleanup rule"
Cohesion: 0.29
Nodes (6): GET /admin/owner-requests/:id/documents/:doc_id, ARCHITECTURE.md, ON DELETE CASCADE — relational-only cleanup rule, listings.price missing index (known gap R-07), listings/search index checklist (idx_listings_*), Sensitive columns exclusion (identity_data, identity_documents JSONB)

### Community 74 - "avatarValidation.ts"
Cohesion: 0.28
Nodes (10): FORMAT_MESSAGE, GENERIC_ERROR_MESSAGE, preCheck(), PROBLEM_MESSAGES, serverMessage(), SIZE_MESSAGE, AvatarUpload(), clearSelection() (+2 more)

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

### Community 82 - "MH-59 — Écran de gestion des photos d'un bien : plan d'implémentation (FE)"
Cohesion: 0.11
Nodes (17): 1. Contexte et objectif, 2.1 Extraction du pré-contrôle et dropzone, 2.2 API et logique pure, 2. Fichiers, dans l'ordre (un fichier à la fois), 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité, 8. Hors périmètre (+9 more)

### Community 83 - "db.rs"
Cohesion: 0.29
Nodes (7): connect_db(), MAX_CONNECTIONS, Error, PgPool, Result, sqlx::PgPool connection pooling, pgpooloptions

### Community 84 - "/graphify"
Cohesion: 0.20
Nodes (9): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Usage (+1 more)

### Community 85 - "useListingPhotoMutations.ts"
Cohesion: 0.22
Nodes (17): 2.3 MH-60-FE : fichiers, dans l'ordre (un fichier à la fois), 4. Points d'intégration avec l'existant, 9. Critères de fin, deleteListingPhoto(), reorderListingPhotos(), setListingCover(), uploadListingPhoto(), LISTING_PHOTOS_MUTATION_KEY (+9 more)

### Community 86 - "route.rs"
Cohesion: 0.19
Nodes (13): apidoc, appenv, ApiDoc, build_router(), merged_router(), openapi_spec(), build_cors_layer, modules (+5 more)

### Community 87 - "owner_requests table"
Cohesion: 0.40
Nodes (5): owner request atomic submission, owner_request_status enum, owner_requests table, PATCH /admin/owner-requests/:id, POST /owner-requests

### Community 88 - "client.ts"
Cohesion: 0.18
Nodes (18): Décisions prises, AccessTokenGetter, apiGet(), apiGetBlob(), apiPatch(), apiPost(), buildQueryString(), ErrorEnvelope (+10 more)

### Community 89 - "ListingPhotosPage.tsx"
Cohesion: 0.09
Nodes (36): Décisions de conception prises pendant l'analyse (sans question), 4. Points d'intégration avec l'existant, 2.3 MH-56-FE — Fichiers, dans l'ordre (un fichier à la fois), 2.3 Composants et intégration, AuthLayout(), RequireAdmin(), RequireAdminProps, RequireAuthProps (+28 more)

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

### Community 101 - "ListingForm.tsx"
Cohesion: 0.13
Nodes (23): ListingDetail, ListingFormMode, ListingFormProps, MODE_TEXT, TYPE_OPTIONS, DESCRIPTION_MAX_LENGTH, DESCRIPTION_MIN_LENGTH, FORM_FIELD_NAMES (+15 more)

### Community 107 - "RateLimitState"
Cohesion: 0.27
Nodes (10): middleware_passes_then_rejects_with_429_and_retry_after(), peer(), RateLimitState, Arc, AtomicU32, IpAddr, IpNet, Self (+2 more)

### Community 108 - "@tanstack/react-query"
Cohesion: 0.08
Nodes (37): 9. Critères de fin, 2.4 MH-57-FE : tests unitaires (vitest, logique pure), 1. Contexte et objectif, 2.3 MH-58-FE : fichiers, dans l'ordre (un fichier à la fois), 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant, 5. Vérifications, 6. Risques et inconnues (+29 more)

### Community 109 - "auth/api.ts"
Cohesion: 0.16
Nodes (11): OwnerRequestStatus, ownerRequestStatusBadge(), STATUS_BADGE, OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken, RefreshResponse, RegisterPayload (+3 more)

### Community 110 - "listings/index.ts"
Cohesion: 0.09
Nodes (40): 4.2 Fichiers, dans l'ordre (un fichier à la fois), 2.3 MH-54-FE — Fichiers, dans l'ordre (un fichier à la fois), Décisions tranchées, 2.1 MH-55-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-55-BE — Tests unitaires (logique pure, sans DB), 2.3 MH-55-FE — Fichiers, dans l'ordre (un fichier à la fois), 2.4 MH-55-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances (+32 more)

### Community 111 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 112 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 125 - "media_order.rs"
Cohesion: 0.26
Nodes (12): non_cover_photo_of_a_three_photo_listing_is_promoted(), photo(), promoting_the_current_cover_changes_nothing(), cover_comes_first_even_with_the_highest_position(), display_order_key(), equal_position_and_upload_time_fall_back_to_id(), equal_positions_put_the_oldest_upload_first(), other_photos_follow_ascending_position() (+4 more)

### Community 158 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 161 - "searchCriteria.ts"
Cohesion: 0.07
Nodes (62): 1. Contexte et objectif, 2. Fichiers, dans l'ordre (un fichier à la fois), 3. Tests unitaires (vitest, logique pure), 4. Points d'intégration avec l'existant, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité, 8. Hors périmètre (+54 more)

### Community 162 - "MH-66 — Libellés de type anglais dans l'index de recherche : plan d'implémentation (BE)"
Cohesion: 0.15
Nodes (12): 1. Contexte et objectif, 2. Fichiers, dans l'ordre (un fichier à la fois), 3. Tests unitaires, 4. Points d'intégration avec l'existant, 5. Risques et inconnues, 6. Vérifications, 7. Contrat qualité, 8. Hors périmètre (+4 more)

### Community 163 - "vitest"
Cohesion: 0.33
Nodes (3): getPageItems(), PageItem, vitest

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

### Community 175 - "listingPhotos.ts"
Cohesion: 0.19
Nodes (16): 3. Tests unitaires (vitest, logique pure, sans DB ni réseau), FORMAT_MESSAGE, freePhotoSlots(), GENERIC_UPLOAD_MESSAGE, isCoverRequiredError(), isMediaOrderConflict(), LISTING_GONE_MESSAGE, MAX_IMAGE_SIZE_MB (+8 more)

### Community 184 - "MH-60 — Réordonner les photos d'un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.17
Nodes (11): 1. Contexte et objectif, 2.1 MH-60-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-60-BE : tests unitaires (logique pure, sans DB), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications, 7. Contrat qualité (sur chaque branche, BE puis FE), 8. Hors périmètre (+3 more)

### Community 185 - "cors.rs"
Cohesion: 0.67
Nodes (3): build_cors_layer(), CorsLayer, http

### Community 186 - "media/handler.rs"
Cohesion: 0.26
Nodes (16): delete_media(), FILE_FIELD_NAME, LISTING_ID_FIELD_NAME, multipart_error(), promote_cover(), read_submission(), reorder_media(), Json (+8 more)

### Community 187 - "rbac.rs"
Cohesion: 0.22
Nodes (5): failed_check_maps_to_forbidden(), require_role(), Role, Result, serde

### Community 188 - "errors.rs"
Cohesion: 0.24
Nodes (15): parse_envelope(), Response, StatusCode, Value, test_bad_request_carries_detail(), test_errors_other_than_validation_omit_fields(), test_internal_error_is_500(), test_listing_not_found_produces_correct_envelope() (+7 more)

### Community 189 - "ListingPhotosPage"
Cohesion: 0.21
Nodes (9): 2.4 MH-60-FE : tests unitaires (vitest, logique pure), isNotFound(), ListingPhotosPage(), dropPendingUpload(), handleFilesSelected(), handleMove(), startUpload(), skippedMessage() (+1 more)

### Community 190 - "logging.rs"
Cohesion: 0.15
Nodes (16): generates_a_valid_uuid_when_header_is_absent(), HEALTH_PATHS, propagates_incoming_x_request_id_header_verbatim(), request_id(), REQUEST_ID_HEADER, resolve_request_id(), HeaderMap, Next (+8 more)

### Community 191 - "What You Must Do When Invoked"
Cohesion: 0.20
Nodes (10): Step 0 - GitHub repos and multi-path merge (only if a URL or several paths), Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio (only if video files detected), Step 4.5 - Graph health check (read-only integrity gate), Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault (opt-in) + HTML, Step 9 - Save manifest, update cost tracker, clean up, and report (+2 more)

### Community 192 - "What You Must Do When Invoked"
Cohesion: 0.20
Nodes (10): Step 0 - GitHub repos and multi-path merge (only if a URL or several paths), Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio (only if video files detected), Step 4.5 - Graph health check (read-only integrity gate), Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault (opt-in) + HTML, Step 9 - Save manifest, update cost tracker, clean up, and report (+2 more)

### Community 193 - "Toast.tsx"
Cohesion: 0.14
Nodes (14): App(), Providers(), ProvidersProps, queryClient, router, frontend_src_index, ToastContext, ToastContextValue (+6 more)

### Community 194 - "owner-request/api.ts"
Cohesion: 0.15
Nodes (20): BadgeSpec, getOwnerRequestStatus(), OwnerRequest, OwnerRequestResponse, ownerRequestStatusQueryKey, OwnerRequestStatusResponse, submitOwnerRequest(), OwnerRequestStatus() (+12 more)

### Community 195 - "Step 3 - Extract entities and relationships"
Cohesion: 0.50
Nodes (4): Part A - Structural extraction for code files, Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction, Step 3 - Extract entities and relationships

### Community 196 - "Step 3 - Extract entities and relationships"
Cohesion: 0.50
Nodes (4): Part A - Structural extraction for code files, Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction, Step 3 - Extract entities and relationships

### Community 198 - "image"
Cohesion: 0.67
Nodes (3): Step 2 - Detect files, Step 2 - Detect files, image()

### Community 199 - "react"
Cohesion: 0.11
Nodes (26): 2.2 MH-64-BE : tests unitaires (logique pure, sans DB), 2.4 MH-64-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, Décisions prises, SearchPage, DocumentViewer(), FilterPanel() (+18 more)

### Community 200 - "ApiError"
Cohesion: 0.18
Nodes (10): 4.1 Préalable, 4.3 Tests unitaires (vitest, logique pure), 4.4 Vérifications FE, 4. MH-53-FE — Formulaire « Publish a listing », 6. Risques et inconnues, 2.3 MH-57-FE : fichiers, dans l'ordre (un fichier à la fois), ListingStatusDto, normalizePlaceName() (+2 more)

### Community 201 - "OwnerRequestDetail"
Cohesion: 0.27
Nodes (8): ReviewDecision, reviewOwnerRequest(), OwnerRequestDetail(), handleApprove(), handleRejectConfirm(), ReviewOwnerRequestInput, useReviewOwnerRequest(), reviewErrorMessage()

### Community 202 - "AuthContext.tsx"
Cohesion: 0.21
Nodes (12): refreshSession(), AuthContext, AuthProvider(), SessionRole, decodeBase64Url(), readTokenRole(), Role, ROLES (+4 more)

### Community 203 - "main.rs"
Cohesion: 0.33
Nodes (5): arc, backend_my_house, duration, socketaddr, tracing_subscriber

### Community 204 - "rate_limit"
Cohesion: 0.33
Nodes (6): rate_limit(), Next, Request, Response, State, ConnectInfo

### Community 205 - "ErrorBody"
Cohesion: 0.40
Nodes (6): ErrorBody, ErrorEnvelope, FieldError, Option, String, Vec

### Community 206 - "ListingForm"
Cohesion: 0.47
Nodes (6): 2.4 MH-56-FE — Tests unitaires (vitest, logique pure), ListingForm(), onSubmit(), listingDetailToFormValues(), serverFieldToFormField(), toListingPayload()

### Community 210 - "super"
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
- **518 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+513 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 936 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **67 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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
- **Why does `AppError` connect `AppError` to `listings/service.rs`, `owner_requests/service.rs`, `AppCacheProvider`, `Mailer`, `auth/repository.rs`, `jwt.rs`, `local_fs.rs`, `users/repository.rs`, `extractors.rs`, `validation.rs`, `owner_requests/repository.rs`, `auth/handler.rs`, `media/service.rs`, `AuthUser`, `users/service.rs`, `Result`, `file_validation.rs`, `users/handler.rs`, `RecordingStorage`, `owner_requests/handler.rs`, `media/repository.rs`, `media/handler.rs`, `rbac.rs`, `errors.rs`, `Result`, `listings/repository.rs`, `ErrorBody`?**
  _High betweenness centrality (0.297) - this node is a cross-community bridge._
- **Why does `4. Points d'intégration avec l'existant` connect `listings/api.ts` to `AppError`, `listings/repository.rs`, `components/index.ts`, `listings/index.ts`, `Alert`, `client.ts`, `ListingPhotosPage.tsx`?**
  _High betweenness centrality (0.144) - this node is a cross-community bridge._
- **Why does `place_filter()` connect `AppError` to `validation.rs`, `listings/api.ts`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._