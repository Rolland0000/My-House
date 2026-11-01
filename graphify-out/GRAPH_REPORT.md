# Graph Report - My-House  (2026-11-01)

## Corpus Check
- 377 files · ~218,178 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 24 file(s) not represented in the graph (top: (none) 15, .toml 5, .example 2)

## Summary
- 2867 nodes · 6840 edges · 182 communities (137 shown, 45 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 552 edges (avg confidence: 0.93)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3a169e1c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- listings/service.rs
- owner_requests/dto.rs
- listings/index.ts
- config/mod.rs
- owner_requests/service.rs
- super
- AppCacheProvider
- auth/service.rs
- components/index.ts
- Mailer
- auth/repository.rs
- MyHouse Project Instructions (Agents)
- jwt.rs
- local_fs.rs
- users/repository.rs
- format.ts
- extractors.rs
- validation.rs
- ListingFeed
- ownerRequestValidation.ts
- admin/api.ts
- AppState
- compilerOptions
- TECHNICAL_SPEC_MVP.md
- ListingStatus
- devDependencies
- PublishListingAction.tsx
- PriceRangeFilter.tsx
- MH-53 — Créer un bien : plan d'implémentation (BE + FE)
- compilerOptions
- auth/handler.rs
- 20260729041038_baseline_schema.sql
- ListingPhotosPage.tsx
- media/service.rs
- admin/handler.rs
- .mcp.json
- My House — Software Architecture Document
- Design Handoff README
- Result
- /graphify
- pagination.rs
- users/handler.rs
- dependencies
- MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)
- My House — Technical Specification MVP
- RecordingStorage
- owner_requests/handler.rs
- rate_limit.rs
- scripts
- ProfileForm.tsx
- users/service.rs
- AppError
- MH-57 — Changer le statut d'un bien : plan d'implémentation (BE + FE)
- CI Backend Workflow
- app_server.rs
- auth/dto.rs
- react
- sqlx::query! / query_as! compile-time checked queries
- MH-67 — Lien vers les autres annonces d'un owner : plan d'implémentation (FE)
- OwnerListingRow.tsx
- mh-15-owner-request.html wireframe
- async_trait
- backend-dev service
- OwnerBar
- 4. Contrat API
- listings/handler.rs
- Docker Rules Skill
- health.rs
- types.rs
- package.json
- listings/repository.rs
- CI Job: Frontend OpenAPI Codegen
- CLAUDE.md — MyHouse project instructions
- ARCHITECTURE.md
- MH-59 — Écran de gestion des photos d'un bien : plan d'implémentation (FE)
- graphify reference: extra exports and benchmark
- K
- 8. Concepts Transverses
- OtpCodeInput
- Owner Request Rejected Email Template
- Embedded React/TypeScript Rules Content
- graphify reference: extra exports and benchmark
- OwnerRequestForm.tsx
- db.rs
- /graphify
- RefreshTokenLookup
- route.rs
- 7. Infrastructure Docker
- client.ts
- router.tsx
- sqlx
- graphify reference: query, path, explain
- mh-12-auth-flow.html wireframe
- analyze job (rust + javascript-typescript matrix)
- Root Docker Compose
- pre-tool-use.sh
- MyHouse Backend Review Invariants
- mh-13-feed-detail.html wireframe
- tsconfig.json
- README.md — Project Overview and Setup
- graphify reference: query, path, explain
- ListingForm.tsx
- MyHouse Frontend Review Invariants
- 1. Introduction et Objectifs
- 5. Vue des Blocs Fonctionnels
- 6. Vue Runtime — Flux Critiques
- 3bis. Stack de Notification
- RateLimitState
- listings/api.ts
- MH-55 — Les biens du propriétaire (« My properties ») : plan d'implémentation (BE + FE)
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- pre-commit
- graphify Slash Command Trigger (.claude/CLAUDE.md)
- Backend Review Checklist Reference
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
- media/handler.rs
- rbac.rs
- errors.rs
- What You Must Do When Invoked
- What You Must Do When Invoked
- Toast.tsx
- owner-request/api.ts
- Step 3 - Extract entities and relationships
- Step 3 - Extract entities and relationships
- image
- FilterPanel.tsx
- listingFormValidation.test.ts
- auth/index.ts
- main.rs
- rate_limit
- ErrorBody

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
- `9. Critères de fin` --references--> `listListings()`  [INFERRED]
  docs/plans/MH-63-plan.md → frontend/src/features/listings/api.ts
- `9. Critères de fin` --references--> `updateListingStatus()`  [INFERRED]
  docs/plans/MH-57-plan.md → frontend/src/features/listings/api.ts
- `9. Critères de fin` --references--> `reorderListingPhotos()`  [INFERRED]
  docs/plans/MH-60-plan.md → frontend/src/features/listings/api.ts
- `8. Hors périmètre` --references--> `ListingFeed()`  [INFERRED]
  docs/plans/MH-64-plan.md → frontend/src/features/listings/components/ListingFeed.tsx
- `3.2 Fichiers, dans l'ordre (un fichier à la fois)` --references--> `ListingNotFound()`  [INFERRED]
  docs/plans/MH-53_IMPLEMENTATION_PLAN.md → frontend/src/features/listings/components/ListingNotFound.tsx

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Frontend CI Pipeline Gate Sequence** — github_workflows_ci_frontend_frontend_install, github_workflows_ci_frontend_frontend_codegen, github_workflows_ci_frontend_frontend_lint, github_workflows_ci_frontend_frontend_test, github_workflows_ci_frontend_frontend_build [EXTRACTED 0.90]
- **Proposed Shared Component Library Spec** — frontend_docs_frontend_design_handoff_my_house_readme_siteheader, frontend_docs_frontend_design_handoff_my_house_readme_sitefooter, frontend_docs_frontend_design_handoff_my_house_readme_badge, frontend_docs_frontend_design_handoff_my_house_readme_dimensionrule, frontend_docs_frontend_design_handoff_my_house_readme_emptystate, frontend_docs_frontend_design_handoff_my_house_readme_skeleton [EXTRACTED 0.90]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **Explicit-sequencing cleanup pattern for account/file deletion** — claude_rules_database_cascade_cleanup, claude_rules_database_storageprovider_delete, docs_architecture [EXTRACTED 1.00]
- **Safe, checked query construction pattern** — claude_rules_database_query_macros, claude_rules_database_querybuilder, claude_rules_database_sql_injection_prevention [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin [EXTRACTED 1.00]
- **Notifications Module Email Template Set** — backend_src_modules_notifications_templates_otp, backend_src_modules_notifications_templates_welcome, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.80]
- **Shared Four-Phase Review Structure** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_code_review_backend_skill_four_phase_review_process, claude_skills_code_review_frontend_skill_four_phase_review_process [INFERRED 0.85]
- **Dev Environment Docker Compose Stack** — backend_compose_backend_backend_dev, frontend_compose_frontend_frontend_dev, docker_compose_db, docker_compose_mailhog [INFERRED 0.85]
- **Dimension Rule Signature Device Across Design Docs** — docs_wireframes_design_tokens_dimension_rule, frontend_docs_frontend_design_handoff_my_house_readme_dimensionrule, frontend_docs_frontend_design_handoff_my_house_my_house_plan_laiton_dc [INFERRED 0.85]
- **Skills Deferring to ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_github_ticket_skill_github_ticket, docs_architecture_doc, docs_technical_spec_mvp [INFERRED 0.85]
- **MyHouse Locked Decisions Enforced Across Project Instructions and Review Skills** — agents_rules_insrtruction_for_my_house_key_decisions_locked, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Owner Request Approval Notification Flow** — backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.85]
- **Refresh Token Handling Invariants Across Review Skills** — claude_skills_code_review_backend_skill_refresh_token_rotation_invariant, claude_skills_code_review_frontend_skill_refresh_token_cookie_invariant, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]

## Communities (182 total, 45 thin omitted)

### Community 0 - "listings/service.rs"
Cohesion: 0.06
Nodes (73): a_draft_summary_serializes_published_at_as_null(), a_published_summary_keeps_its_published_at(), ListingDetailDto, ListingMediaDto, ListingRequest, ListingStatusDto, ListingStatusRequest, ListingStatusResponse (+65 more)

### Community 1 - "owner_requests/dto.rs"
Cohesion: 0.07
Nodes (63): admin_detail_dto_falls_back_to_empty_documents_on_malformed_json(), admin_detail_dto_never_exposes_storage_key(), admin_detail_row(), admin_list_dto_omits_identity_data_and_documents(), admin_row(), AdminOwnerRequestDetailDto, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocumentDto (+55 more)

### Community 2 - "listings/index.ts"
Cohesion: 0.14
Nodes (25): 2.3 MH-55-FE — Fichiers, dans l'ordre (un fichier à la fois), Décisions de conception prises pendant l'analyse, 4. Points d'intégration avec l'existant, 2.3 MH-63-FE : fichiers, dans l'ordre (un fichier à la fois), ListingFeed, getListing(), ListingSummary, listListings() (+17 more)

### Community 3 - "config/mod.rs"
Cohesion: 0.07
Nodes (67): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, dev_allows_smtp_credentials_over_plaintext(), dev_without_smtp_credentials_sends_plaintext_unauthenticated(), empty_smtp_credentials_count_as_absent() (+59 more)

### Community 4 - "owner_requests/service.rs"
Cohesion: 0.08
Nodes (43): a_doc_id_from_a_different_request_does_not_resolve(), a_missing_storage_object_surfaces_as_document_not_found(), a_pdf_renamed_into_an_image_slot_is_a_shape_violation(), accepts_one_pdf_with_no_side(), accepts_two_images_as_front_and_back(), as_document_not_found(), as_shape_error(), ClassifiedDocument (+35 more)

### Community 5 - "super"
Cohesion: 0.05
Nodes (46): a_missing_or_failing_key_does_not_abort_the_remaining_keys(), delete_storage_objects(), deletes_every_enumerated_key(), no_keys_is_a_no_op(), StorageProvider, String, generates_a_valid_uuid_when_header_is_absent(), HEALTH_PATHS (+38 more)

### Community 6 - "AppCacheProvider"
Cohesion: 0.17
Nodes (25): AppCache, AUTH_CHALLENGE_MAX_ENTRIES, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache(), IP_RATE_LIMIT_MAX_ENTRIES (+17 more)

### Community 7 - "auth/service.rs"
Cohesion: 0.19
Nodes (25): handle_revoked(), logout(), mint(), otp_request(), OtpVerifyConfig, refresh(), register(), RegisterInput (+17 more)

### Community 8 - "components/index.ts"
Cohesion: 0.05
Nodes (65): 2. Existant sur lequel on s'appuie, 4.2 Fichiers, dans l'ordre (un fichier à la fois), 4. Points d'intégration avec l'existant, STATUS_OPTIONS, OwnerFilterChip(), OwnerFilterChipProps, SearchBarProps, Alert() (+57 more)

### Community 9 - "Mailer"
Cohesion: 0.05
Nodes (55): Address, AddressError, AsyncSmtpTransport, authenticates_with_configured_credentials(), builds_successfully_with_valid_config(), builds_with_credentials_and_each_security_mode(), credentials(), hello_name() (+47 more)

### Community 10 - "auth/repository.rs"
Cohesion: 0.33
Nodes (15): create_account(), db_err(), email_exists(), find_by_hash(), find_user_by_email(), insert_refresh_token(), revoke(), revoke_all_for_user() (+7 more)

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

### Community 15 - "format.ts"
Cohesion: 0.13
Nodes (21): 1. Contexte et objectif, 2.1 Fichiers, dans l'ordre (un fichier à la fois), 2.2 Ce qui ne change pas, 2.3 Vérification, 2. Points d'intégration avec l'existant, 4. Contrat qualité (méthodologie d'implémentation), Décisions prises, MH-62 — Page détail alignée sur DET-01 / DET-02 : plan d'implémentation (FE) (+13 more)

### Community 16 - "extractors.rs"
Cohesion: 0.11
Nodes (28): AppJson<T>, AppQuery<T>, AuthState, AuthUser, bearer_token(), MaybeAuthUser, optional_bearer_token(), resolve_identity() (+20 more)

### Community 17 - "validation.rs"
Cohesion: 0.07
Nodes (28): FieldErrors, finish_reports_one_entry_per_recorded_violation(), keyword_filter(), keyword_filter_counts_characters_not_bytes(), MAX_ADMIN_NOTE_LENGTH, MAX_KEYWORD_LENGTH, MAX_NAME_LENGTH, MAX_PHONE_LENGTH (+20 more)

### Community 18 - "ListingFeed"
Cohesion: 0.11
Nodes (21): 1. Contexte et objectif, 2.1 MH-63-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-63-BE : tests unitaires (logique pure, sans DB), 2.4 MH-63-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant, 5. Vérifications (+13 more)

### Community 19 - "ownerRequestValidation.ts"
Cohesion: 0.23
Nodes (11): OwnerRequestForm(), GENERIC_ERROR_MESSAGE, IMAGE_FORMAT_MESSAGE, IMAGE_SIZE_MESSAGE, PDF_FORMAT_MESSAGE, PDF_SIZE_MESSAGE, preCheckImage(), preCheckPdf() (+3 more)

### Community 20 - "admin/api.ts"
Cohesion: 0.16
Nodes (20): AdminOwnerRequest, AdminOwnerRequestDetail, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocument, getOwnerRequest(), getOwnerRequestDocument(), listOwnerRequests(), ListOwnerRequestsParams (+12 more)

### Community 21 - "AppState"
Cohesion: 0.09
Nodes (31): AppState, Inner, Arc, PgPool, Self, StorageProvider, router(), OpenApiRouter (+23 more)

### Community 22 - "compilerOptions"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+11 more)

### Community 23 - "TECHNICAL_SPEC_MVP.md"
Cohesion: 0.14
Nodes (18): fn_set_updated_at trigger, Partial unique indexes for 'at most one active X' invariants, Repository tests in rolled-back transaction (integration feature flag), Schema conventions (UUID PK, timestamps, enums), Backend Review Checklist (reference/checklist.md), Code Review — Backend Skill, Four-Phase Backend Review Process, Backend Review Severity Labels (+10 more)

### Community 24 - "ListingStatus"
Cohesion: 0.13
Nodes (23): 1. Contexte et objectif, 2.1 MH-64-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-64-BE : tests unitaires (logique pure, sans DB), 2.3 MH-64-FE : fichiers, dans l'ordre (un fichier à la fois), 2. Découpage BE / FE et dépendances, 5. Vérifications, 7. Contrat qualité (sur chaque branche, BE puis FE), 8. Hors périmètre (+15 more)

### Community 25 - "devDependencies"
Cohesion: 0.11
Nodes (18): devDependencies, eslint, @eslint/js, eslint-plugin-react-hooks, eslint-plugin-react-refresh, globals, openapi-typescript, prettier (+10 more)

### Community 26 - "PublishListingAction.tsx"
Cohesion: 0.12
Nodes (18): 2.1 MH-61-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-61-BE : tests unitaires (logique pure, sans DB), 2.4 MH-61-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (sur chaque branche, BE puis FE) (+10 more)

### Community 27 - "PriceRangeFilter.tsx"
Cohesion: 0.22
Nodes (21): 3. Tests unitaires (vitest, logique pure), PRICE_MAX, PriceRangeFilter(), clampToOtherHandle(), commit(), commitHandle(), handleFieldKeyDown(), handleRangeKeyDown() (+13 more)

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

### Community 32 - "ListingPhotosPage.tsx"
Cohesion: 0.08
Nodes (43): 3. Tests unitaires (vitest, logique pure, sans DB ni réseau), 2.3 MH-60-FE : fichiers, dans l'ordre (un fichier à la fois), 2.4 MH-60-FE : tests unitaires (vitest, logique pure), 4. Points d'intégration avec l'existant, 6. Risques et inconnues, ListingPhotosPage, deleteListingPhoto(), ListingMedia (+35 more)

### Community 33 - "media/service.rs"
Cohesion: 0.05
Nodes (63): Self, MediaForDeletion, MediaRow, OrderedMediaRow, Bytes, String, Uuid, UploadMediaSubmission (+55 more)

### Community 34 - "admin/handler.rs"
Cohesion: 0.26
Nodes (15): AdminOwnerRequestDetailResponse, get_owner_request(), get_owner_request_document(), list_owner_requests(), review_owner_request(), IntoResponse, Json, Path (+7 more)

### Community 35 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 36 - "My House — Software Architecture Document"
Cohesion: 0.09
Nodes (23): 10.1 Objectifs et Scénarios, 10.2 Arbre de Qualité, 10. Exigences de Qualité, 11. Risques et Dette Technique, 12. Glossaire, 2.1 Contraintes Techniques, 2.2 Contraintes Organisationnelles, 2. Contraintes (+15 more)

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
Cohesion: 0.26
Nodes (15): AVATAR_FIELD_NAME, delete_me(), get_me(), read_file_field(), Bytes, CookieJar, Json, Multipart (+7 more)

### Community 42 - "dependencies"
Cohesion: 0.25
Nodes (8): dependencies, clsx, lucide-react, react, react-dom, react-hook-form, react-router, @tanstack/react-query

### Community 43 - "MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)"
Cohesion: 0.15
Nodes (12): 1. Contexte et objectif, 2.1 MH-54-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-54-BE — Tests unitaires (logique pure, sans DB), 2.4 MH-54-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE) (+4 more)

### Community 44 - "My House — Technical Specification MVP"
Cohesion: 0.10
Nodes (21): 1.1 Backend, 1.2 Frontend, 1. Structure du Projet, 2.1 Schéma Relationnel, 2.2 DDL Complet, 2.3 Notes de Conception, 2. Modèle de Données, 3.1 Trait `StorageProvider` (+13 more)

### Community 45 - "RecordingStorage"
Cohesion: 0.23
Nodes (9): RecordingStorage, Bytes, Duration, Mutex, Result, Self, StorageProvider, String (+1 more)

### Community 46 - "owner_requests/handler.rs"
Cohesion: 0.16
Nodes (17): DOCUMENTS_FIELD_NAME, get_owner_request_status(), read_submission(), Json, Multipart, Result, State, StatusCode (+9 more)

### Community 47 - "rate_limit.rs"
Cohesion: 0.13
Nodes (22): apperror, atomic, allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it(), distinct_clients_get_distinct_counters(), falls_back_to_peer_ip_when_trusted_header_is_missing(), headers_with_xff(), ignores_x_forwarded_for_from_an_untrusted_peer(), NGINX (+14 more)

### Community 48 - "scripts"
Cohesion: 0.15
Nodes (13): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+5 more)

### Community 49 - "ProfileForm.tsx"
Cohesion: 0.11
Nodes (29): 2.3 MH-54-FE — Fichiers, dans l'ordre (un fichier à la fois), RegistrationFormProps, deleteAccount(), getMe(), Profile, profileQueryKey, updateMe(), UpdateProfilePayload (+21 more)

### Community 50 - "users/service.rs"
Cohesion: 0.09
Nodes (33): AvatarUploadForm, response_envelope_serializes_the_profile_fields(), row(), row_maps_to_dto_field_for_field(), From, Option, Role, Self (+25 more)

### Community 51 - "AppError"
Cohesion: 0.17
Nodes (29): multipart_error(), MultipartError, count_media(), db_err(), delete_media(), demote_other_covers(), find_listing_media(), insert_media() (+21 more)

### Community 52 - "MH-57 — Changer le statut d'un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.18
Nodes (10): 1. Contexte et objectif, 3. Ordre d'implémentation recommandé, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (sur chaque branche, BE puis FE), 8. Hors périmètre, 9. Critères de fin, Décisions prises (+2 more)

### Community 53 - "CI Backend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 54 - "app_server.rs"
Cohesion: 0.18
Nodes (11): AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), Box, build_router (+3 more)

### Community 55 - "auth/dto.rs"
Cohesion: 0.26
Nodes (13): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+5 more)

### Community 56 - "react"
Cohesion: 0.09
Nodes (27): AuthFlow, OwnerRequestDetail, DocumentViewer(), requestOtp(), AuthFlow(), leaveRegistration(), AuthStepIndicator(), markInterrupted() (+19 more)

### Community 57 - "sqlx::query! / query_as! compile-time checked queries"
Cohesion: 0.50
Nodes (5): DATABASE_URL compile-time requirement, sqlx::query! / query_as! compile-time checked queries, sqlx QueryBuilder, No string-interpolated SQL rule, .sqlx/ offline cache (cargo sqlx prepare)

### Community 58 - "MH-67 — Lien vers les autres annonces d'un owner : plan d'implémentation (FE)"
Cohesion: 0.19
Nodes (10): 1. Contexte et objectif, 2. Fichiers, dans l'ordre, 3. Tests unitaires (vitest, logique pure), 5. Risques, 6. Vérifications, 7. Contrat qualité, Décisions, MH-67 — Lien vers les autres annonces d'un owner : plan d'implémentation (FE) (+2 more)

### Community 59 - "OwnerListingRow.tsx"
Cohesion: 0.21
Nodes (9): CHECKED_CLASSES, ListingStatusControl(), STATUSES, OwnerBarProps, VISIBILITY_MESSAGE, OwnerListingRow(), VISIBILITY_BADGE, listingVisibility (+1 more)

### Community 60 - "mh-15-owner-request.html wireframe"
Cohesion: 0.20
Nodes (10): mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire'), Identity document dropzone component, Request approved status page (+2 more)

### Community 61 - "async_trait"
Cohesion: 0.17
Nodes (10): async_trait, MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache (+2 more)

### Community 62 - "backend-dev service"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 63 - "OwnerBar"
Cohesion: 0.19
Nodes (16): 3. Ordre d'implémentation recommandé, 1. Contexte et objectif, 2.1 MH-56-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-56-BE — Tests unitaires (logique pure, sans DB), 2.4 MH-56-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 5. Vérifications (+8 more)

### Community 64 - "4. Contrat API"
Cohesion: 0.17
Nodes (13): 4.0 Health Check, 4.1 Auth, 4.2 Users, 4.2bis Owner Requests, 4.3 Listings, 4.4 Search, 4.5 Media, 4.6 Contact (+5 more)

### Community 65 - "listings/handler.rs"
Cohesion: 0.20
Nodes (25): ListingDetailResponse, create(), delete(), get_by_id(), list(), list_mine(), OWNER_ROLES, publish() (+17 more)

### Community 66 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 67 - "health.rs"
Cohesion: 0.21
Nodes (10): axum, check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus (+2 more)

### Community 68 - "types.rs"
Cohesion: 0.22
Nodes (4): AuthChallenge, PendingOtp, String, fmt

### Community 69 - "package.json"
Cohesion: 0.11
Nodes (19): name, private, type, version, clsx, eslint, @eslint/js, eslint-plugin-react-hooks (+11 more)

### Community 70 - "listings/repository.rs"
Cohesion: 0.11
Nodes (44): Self, ListingDetailRow, ListingFields, ListingMediaRow, ListingStatus, ListingSummaryRow, ListingType, Option (+36 more)

### Community 71 - "CI Job: Frontend OpenAPI Codegen"
Cohesion: 0.43
Nodes (8): CI Job: Frontend Build, CI Job: Frontend OpenAPI Codegen, CI Job: Frontend Docker Build, CI Job: Frontend Install, CI Job: Frontend Lint, CI Job: Frontend Prettier, CI Job: Frontend Unit Tests, CI Job: Frontend TypeScript

### Community 72 - "CLAUDE.md — MyHouse project instructions"
Cohesion: 0.29
Nodes (7): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), mcp/.toolbox/tool.yaml — postgres-local MCP toolbox source

### Community 73 - "ARCHITECTURE.md"
Cohesion: 0.29
Nodes (5): GET /admin/owner-requests/:id/documents/:doc_id, ON DELETE CASCADE — relational-only cleanup rule, listings.price missing index (known gap R-07), listings/search index checklist (idx_listings_*), Sensitive columns exclusion (identity_data, identity_documents JSONB)

### Community 74 - "MH-59 — Écran de gestion des photos d'un bien : plan d'implémentation (FE)"
Cohesion: 0.13
Nodes (20): 2.1 Extraction du pré-contrôle et dropzone, 2.2 API et logique pure, 2. Fichiers, dans l'ordre (un fichier à la fois), 5. Vérifications, 6. Risques et inconnues, 8. Hors périmètre, 9. Critères de fin, MH-59 — Écran de gestion des photos d'un bien : plan d'implémentation (FE) (+12 more)

### Community 75 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 76 - "K"
Cohesion: 0.38
Nodes (4): MokaStore<K, V>, K, Option, V

### Community 77 - "8. Concepts Transverses"
Cohesion: 0.25
Nodes (8): 8.1 Authentification et Autorisation, 8.2 Gestion des Erreurs, 8.3 Pagination, 8.4 Logging, 8.5 Configuration, 8.6 Génération des Types TypeScript, 8.7 Health Check et Arrêt Propre (Graceful Shutdown), 8. Concepts Transverses

### Community 78 - "OtpCodeInput"
Cohesion: 0.57
Nodes (7): OtpCodeInput(), focusInput(), handleChange(), handleKeyDown(), handlePaste(), setDigit(), submit()

### Community 79 - "Owner Request Rejected Email Template"
Cohesion: 0.33
Nodes (6): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template, Optional rejection reason, Resubmission allowed after correction, user_name template variable

### Community 80 - "Embedded React/TypeScript Rules Content"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 81 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 82 - "OwnerRequestForm.tsx"
Cohesion: 0.12
Nodes (9): OwnerRequestForm, FileWithPreview, FormValues, ID_TYPE_OPTIONS, ImageSlotProps, Modality, PdfSlotProps, FileDropzone() (+1 more)

### Community 83 - "db.rs"
Cohesion: 0.29
Nodes (7): connect_db(), MAX_CONNECTIONS, Error, PgPool, Result, sqlx::PgPool connection pooling, pgpooloptions

### Community 84 - "/graphify"
Cohesion: 0.20
Nodes (9): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Usage (+1 more)

### Community 85 - "RefreshTokenLookup"
Cohesion: 0.47
Nodes (5): NewAccount, RefreshTokenLookup, Option, Role, Uuid

### Community 86 - "route.rs"
Cohesion: 0.19
Nodes (13): apidoc, appenv, ApiDoc, build_router(), merged_router(), openapi_spec(), build_cors_layer, modules (+5 more)

### Community 87 - "7. Infrastructure Docker"
Cohesion: 0.40
Nodes (5): 7.1 `backend/Dockerfile`, 7.2 `frontend/Dockerfile`, 7.3 `frontend/nginx.conf`, 7.4 `docker-compose.yml`, 7. Infrastructure Docker

### Community 88 - "client.ts"
Cohesion: 0.12
Nodes (26): 4. Points d'intégration avec l'existant, 2.3 MH-61-FE : fichiers, dans l'ordre (un fichier à la fois), OwnerRequestDetail(), handleApprove(), handleRejectConfirm(), reviewErrorMessage(), AccessTokenGetter, apiDelete() (+18 more)

### Community 89 - "router.tsx"
Cohesion: 0.09
Nodes (42): Décisions de conception prises pendant l'analyse (sans question), 4. Points d'intégration avec l'existant, Décisions tranchées, 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant, 2.3 MH-56-FE — Fichiers, dans l'ordre (un fichier à la fois), 4. Points d'intégration avec l'existant, 2.3 Composants et intégration (+34 more)

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

### Community 96 - "MyHouse Backend Review Invariants"
Cohesion: 0.50
Nodes (4): AuthUser is_active Re-verification Invariant, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, StorageProvider Abstraction Requirement

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
Cohesion: 0.17
Nodes (18): ListingDetail, ListingFormMode, ListingFormProps, MODE_TEXT, TYPE_OPTIONS, DESCRIPTION_MAX_LENGTH, DESCRIPTION_MIN_LENGTH, FORM_FIELD_NAMES (+10 more)

### Community 102 - "MyHouse Frontend Review Invariants"
Cohesion: 0.50
Nodes (4): types.ts Generated File Invariant, MyHouse Frontend Review Invariants, Refresh Token httpOnly Cookie Invariant, Role-Gated Route Guard Invariant

### Community 103 - "1. Introduction et Objectifs"
Cohesion: 0.50
Nodes (4): 1.1 Objectif du Système, 1.2 Objectifs de Qualité, 1.3 Parties Prenantes, 1. Introduction et Objectifs

### Community 104 - "5. Vue des Blocs Fonctionnels"
Cohesion: 0.50
Nodes (4): 5.1 Niveau 1 — Vue Système, 5.2 Niveau 2 — Modules de Domaine, 5.3 Dépendances Inter-Modules, 5. Vue des Blocs Fonctionnels

### Community 105 - "6. Vue Runtime — Flux Critiques"
Cohesion: 0.50
Nodes (4): 6.1 Authentification OTP — Login et Inscription Unifiés, 6.2 Demande et Validation du Rôle Owner, 6.3 Core Loop — Publication et Découverte d'un Bien, 6. Vue Runtime — Flux Critiques

### Community 106 - "3bis. Stack de Notification"
Cohesion: 0.50
Nodes (4): 3bis.1 Chaîne technique (MVP), 3bis.2 Événements déclencheurs (MVP), 3bis.3 Hors scope MVP (déféré V2), 3bis. Stack de Notification

### Community 107 - "RateLimitState"
Cohesion: 0.27
Nodes (10): middleware_passes_then_rejects_with_429_and_retry_after(), peer(), RateLimitState, Arc, AtomicU32, IpAddr, IpNet, Self (+2 more)

### Community 108 - "listings/api.ts"
Cohesion: 0.08
Nodes (38): 1. Contexte et objectif, 2.2 MH-58-BE : tests unitaires (logique pure, sans DB), 2.3 MH-58-FE : fichiers, dans l'ordre (un fichier à la fois), 2.4 MH-58-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 3. Ordre d'implémentation recommandé, 4. Points d'intégration avec l'existant, 5. Vérifications (+30 more)

### Community 110 - "MH-55 — Les biens du propriétaire (« My properties ») : plan d'implémentation (BE + FE)"
Cohesion: 0.12
Nodes (15): 1. Contexte et objectif, 2.1 MH-55-BE — Fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-55-BE — Tests unitaires (logique pure, sans DB), 2.4 MH-55-FE — Tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE) (+7 more)

### Community 111 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 112 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 158 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 161 - "searchCriteria.ts"
Cohesion: 0.11
Nodes (35): 2.4 MH-64-FE : tests unitaires (vitest, logique pure), 3. Ordre d'implémentation recommandé, 2. Fichiers, dans l'ordre (un fichier à la fois), 4. Points d'intégration avec l'existant, 5. Vérifications, 6. Risques et inconnues, 7. Contrat qualité, 8. Hors périmètre (+27 more)

### Community 162 - "MH-66 — Libellés de type anglais dans l'index de recherche : plan d'implémentation (BE)"
Cohesion: 0.15
Nodes (12): 1. Contexte et objectif, 2. Fichiers, dans l'ordre (un fichier à la fois), 3. Tests unitaires, 4. Points d'intégration avec l'existant, 5. Risques et inconnues, 6. Vérifications, 7. Contrat qualité, 8. Hors périmètre (+4 more)

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
Cohesion: 0.13
Nodes (20): DeletePhotoModal(), DeletePhotoModalProps, COVER_REQUIRED_MESSAGE, FORMAT_MESSAGE, GENERIC_UPLOAD_MESSAGE, isCoverRequiredError(), LISTING_GONE_MESSAGE, MAX_IMAGE_SIZE_MB (+12 more)

### Community 184 - "MH-60 — Réordonner les photos d'un bien : plan d'implémentation (BE + FE)"
Cohesion: 0.10
Nodes (20): 1. Décisions tranchées, 2.1 MH-57-BE : fichiers, dans l'ordre (un fichier à la fois), 2.2 MH-57-BE : tests unitaires (logique pure, sans DB), 2.3 MH-57-FE : fichiers, dans l'ordre (un fichier à la fois), 2.4 MH-57-FE : tests unitaires (vitest, logique pure), 2. Découpage BE / FE et dépendances, 1. Contexte et objectif, 2.1 MH-60-BE : fichiers, dans l'ordre (un fichier à la fois) (+12 more)

### Community 186 - "media/handler.rs"
Cohesion: 0.15
Nodes (26): MediaDto, MediaListResponse, MediaResponse, PromoteCoverRequest, ReorderMediaRequest, From, Option, String (+18 more)

### Community 187 - "rbac.rs"
Cohesion: 0.22
Nodes (5): failed_check_maps_to_forbidden(), require_role(), Role, Result, serde

### Community 188 - "errors.rs"
Cohesion: 0.24
Nodes (15): parse_envelope(), Response, StatusCode, Value, test_bad_request_carries_detail(), test_errors_other_than_validation_omit_fields(), test_internal_error_is_500(), test_listing_not_found_produces_correct_envelope() (+7 more)

### Community 191 - "What You Must Do When Invoked"
Cohesion: 0.20
Nodes (10): Step 0 - GitHub repos and multi-path merge (only if a URL or several paths), Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio (only if video files detected), Step 4.5 - Graph health check (read-only integrity gate), Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault (opt-in) + HTML, Step 9 - Save manifest, update cost tracker, clean up, and report (+2 more)

### Community 192 - "What You Must Do When Invoked"
Cohesion: 0.20
Nodes (10): Step 0 - GitHub repos and multi-path merge (only if a URL or several paths), Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio (only if video files detected), Step 4.5 - Graph health check (read-only integrity gate), Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault (opt-in) + HTML, Step 9 - Save manifest, update cost tracker, clean up, and report (+2 more)

### Community 193 - "Toast.tsx"
Cohesion: 0.11
Nodes (17): App(), Providers(), ProvidersProps, queryClient, router, frontend_src_index, ModalProps, ModalSize (+9 more)

### Community 194 - "owner-request/api.ts"
Cohesion: 0.18
Nodes (17): BadgeSpec, getOwnerRequestStatus(), OwnerRequest, OwnerRequestResponse, ownerRequestStatusQueryKey, OwnerRequestStatusResponse, submitOwnerRequest(), OwnerRequestStatus() (+9 more)

### Community 195 - "Step 3 - Extract entities and relationships"
Cohesion: 0.50
Nodes (4): Part A - Structural extraction for code files, Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction, Step 3 - Extract entities and relationships

### Community 196 - "Step 3 - Extract entities and relationships"
Cohesion: 0.50
Nodes (4): Part A - Structural extraction for code files, Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction, Step 3 - Extract entities and relationships

### Community 198 - "image"
Cohesion: 0.67
Nodes (3): Step 2 - Detect files, Step 2 - Detect files, image()

### Community 199 - "FilterPanel.tsx"
Cohesion: 0.20
Nodes (12): 1. Contexte et objectif, Décisions prises, FilterPanelProps, FilterPatch, statusEntries, TextFilter(), TextFilterProps, toggle() (+4 more)

### Community 200 - "listingFormValidation.test.ts"
Cohesion: 0.15
Nodes (18): 4.1 Préalable, 4.3 Tests unitaires (vitest, logique pure), 4.4 Vérifications FE, 4. MH-53-FE — Formulaire « Publish a listing », 6. Risques et inconnues, Décisions de conception prises pendant l'analyse, ListingForm(), onSubmit() (+10 more)

### Community 202 - "auth/index.ts"
Cohesion: 0.08
Nodes (35): RequireAuthProps, OwnerRequestQueueList(), OwnerRequestStatus, ownerRequestStatusBadge(), STATUS_BADGE, OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken (+27 more)

### Community 203 - "main.rs"
Cohesion: 0.33
Nodes (5): arc, backend_my_house, duration, socketaddr, tracing_subscriber

### Community 204 - "rate_limit"
Cohesion: 0.33
Nodes (6): rate_limit(), Next, Request, Response, State, ConnectInfo

### Community 205 - "ErrorBody"
Cohesion: 0.40
Nodes (6): ErrorBody, ErrorEnvelope, FieldError, Option, String, Vec

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
- **542 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+537 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 942 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **45 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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
- **Why does `AppError` connect `AppError` to `listings/service.rs`, `owner_requests/dto.rs`, `owner_requests/service.rs`, `auth/service.rs`, `Mailer`, `auth/repository.rs`, `jwt.rs`, `local_fs.rs`, `users/repository.rs`, `extractors.rs`, `validation.rs`, `AppState`, `auth/handler.rs`, `media/service.rs`, `admin/handler.rs`, `Result`, `file_validation.rs`, `users/handler.rs`, `RecordingStorage`, `owner_requests/handler.rs`, `users/service.rs`, `media/handler.rs`, `rbac.rs`, `errors.rs`, `listings/handler.rs`, `listings/repository.rs`, `ErrorBody`?**
  _High betweenness centrality (0.310) - this node is a cross-community bridge._
- **Why does `4. Points d'intégration avec l'existant` connect `components/index.ts` to `listings/handler.rs`, `listings/index.ts`, `listings/repository.rs`, `MH-55 — Les biens du propriétaire (« My properties ») : plan d'implémentation (BE + FE)`, `validation.rs`, `ListingFeed`, `ListingStatus`, `router.tsx`?**
  _High betweenness centrality (0.145) - this node is a cross-community bridge._
- **Why does `place_filter()` connect `validation.rs` to `listings/service.rs`, `components/index.ts`, `AppError`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._