# Graph Report - My-House  (2026-09-15)

## Corpus Check
- 267 files · ~138,538 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1763 nodes · 3629 edges · 158 communities (114 shown, 44 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 101 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7812783e`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .new
- users/service.rs
- AppCacheProvider
- components/index.ts
- config/mod.rs
- admin/api.ts
- owner_requests/dto.rs
- jwt.rs
- rate_limit.rs
- owner_requests/service.rs
- UserRow
- Mailer
- MyHouse Project Instructions (Agents)
- ProfileForm.tsx
- local_fs.rs
- ListingFeed.tsx
- OtpVerifyForm.tsx
- auth/index.ts
- compilerOptions
- extractors.rs
- Code Review — Backend Skill
- AppError
- devDependencies
- SiteHeader.tsx
- OwnerRequestDetail.tsx
- compilerOptions
- auth/handler.rs
- client.ts
- TokenDecoder
- admin/handler.rs
- users/handler.rs
- .mcp.json
- AppState
- router.tsx
- owner_requests/repository.rs
- DeleteAccountSection.tsx
- errors.rs
- dependencies
- Module: users
- Design Handoff README
- auth/dto.rs
- Extraction subagent prompt (full)
- admin identity-document read exception
- UnimplementedStorage
- AuthUser
- scripts
- MH-49-BE / MH-49-FE — Admin approve/reject decision on owner requests
- StorageProvider Trait
- resolve_request_id
- Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark
- CI Backend Workflow
- .run
- health.rs
- sqlx::query! / query_as! compile-time checked queries
- save-result feedback loop
- users table
- mh-15-owner-request.html wireframe
- backend-dev service
- MokaStore
- /graphify add <url>
- Docker Rules Skill
- /graphify command
- shared module
- package.json
- Pagination.tsx
- CI Job: Frontend OpenAPI Codegen
- CLAUDE.md — MyHouse project instructions
- MokaStore<K, V>
- ON DELETE CASCADE — relational-only cleanup rule
- Multiple repos cross-repo graph merge
- Step 4 - Build graph, cluster, analyze, generate outputs
- users module
- Auth API endpoints (§4.1)
- providers.tsx
- Embedded React/TypeScript Rules Content
- --update (incremental re-extraction)
- connect_db
- Step 9 - Save manifest, update cost tracker, clean up, and report
- owner_requests table
- generate_otp_code
- sqlx
- git commit hook (graphify hook install)
- mh-12-auth-flow.html wireframe
- analyze job (rust + javascript-typescript matrix)
- Root Docker Compose
- Owner Request Approved Email Template
- pre-tool-use.sh
- Registration Ticket (opaque UUID)
- mh-13-feed-detail.html wireframe
- tsconfig.json
- README.md — Project Overview and Setup
- infra/db.rs
- No-Proxy Principle for Public Files
- R-08: search_vector trigger N+1 query
- Refresh Token httpOnly Cookie
- GET /users/me/owner-request
- eslint-plugin-react-hooks
- globals
- tailwindcss
- @types/react-dom
- typescript-eslint
- vite
- pre-commit
- Extraction subagent prompt (compact)
- useReviewOwnerRequest.ts
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

## God Nodes (most connected - your core abstractions)
1. `AppError` - 136 edges
2. `AppState` - 51 edges
3. `cn()` - 40 edges
4. `AppCacheProvider` - 29 edges
5. `Mailer` - 24 edges
6. `AuthUser` - 21 edges
7. `AppConfig` - 20 edges
8. `set_valid_env()` - 20 edges
9. `AppCache` - 19 edges
10. `Role` - 18 edges

## Surprising Connections (you probably didn't know these)
- `Step B2 - Dispatch ALL subagents in a single message` --semantically_similar_to--> `Step B2 - Dispatch ALL subagents (Codex spawn_agent)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md
- `Extraction subagent prompt (full)` --semantically_similar_to--> `Extraction subagent prompt (compact)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/references/extraction-spec.md → .codex/skills/graphify/references/extraction-spec.md
- `--update (incremental re-extraction)` --semantically_similar_to--> `--update (incremental re-extraction) (Codex)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/references/update.md → .codex/skills/graphify/references/update.md
- `AGENTS.md — graphify trigger instructions` --semantically_similar_to--> `CLAUDE.md — MyHouse project instructions`  [INFERRED] [semantically similar]
  AGENTS.md → CLAUDE.md
- `/graphify command` --semantically_similar_to--> `/graphify command (Codex variant)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Safe, checked query construction pattern** — claude_rules_database_query_macros, claude_rules_database_querybuilder, claude_rules_database_sql_injection_prevention [EXTRACTED 1.00]
- **Explicit-sequencing cleanup pattern for account/file deletion** — claude_rules_database_cascade_cleanup, claude_rules_database_storageprovider_delete, claude_rules_database_architecture_md [EXTRACTED 1.00]
- **MVP notification delivery pipeline** — docs_technical_spec_mvp_v1_2_users_module, docs_technical_spec_mvp_v1_2_admin_module, docs_technical_spec_mvp_v1_2_notifications_service_send_email, docs_technical_spec_mvp_v1_2_infra_mailer, docs_technical_spec_mvp_v1_2_env_vars [EXTRACTED 1.00]
- **storage key naming convention (public vs private prefixes)** — docs_technical_spec_mvp_v1_2_public_listing_storage_key, docs_technical_spec_mvp_v1_2_public_avatar_storage_key, docs_technical_spec_mvp_v1_2_private_owner_request_storage_key [EXTRACTED 1.00]
- **MVP security/data-integrity invariants** — docs_technical_spec_mvp_v1_2_magic_bytes_validation, docs_technical_spec_mvp_v1_2_server_generated_filenames, docs_technical_spec_mvp_v1_2_owner_request_atomic_submission, docs_technical_spec_mvp_v1_2_refresh_token_rotation [INFERRED 0.85]
- **Dimension Rule Signature Device Across Design Docs** — docs_wireframes_design_tokens_dimension_rule, frontend_docs_frontend_design_handoff_my_house_readme_dimensionrule, frontend_docs_frontend_design_handoff_my_house_my_house_plan_laiton_dc [INFERRED 0.85]
- **Proposed Shared Component Library Spec** — frontend_docs_frontend_design_handoff_my_house_readme_siteheader, frontend_docs_frontend_design_handoff_my_house_readme_badge, frontend_docs_frontend_design_handoff_my_house_readme_dimensionrule, frontend_docs_frontend_design_handoff_my_house_readme_emptystate, frontend_docs_frontend_design_handoff_my_house_readme_skeleton [EXTRACTED 0.90]
- **Frontend CI Pipeline Gate Sequence** — github_workflows_ci_frontend_frontend_install, github_workflows_ci_frontend_frontend_codegen, github_workflows_ci_frontend_frontend_lint, github_workflows_ci_frontend_frontend_test, github_workflows_ci_frontend_frontend_build [EXTRACTED 0.90]
- **MyHouse Domain Modules (Modular Monolith)** — docs_architecture_v1_2_module_auth, docs_architecture_v1_2_module_users, docs_architecture_v1_2_module_listings, docs_architecture_v1_2_module_search, docs_architecture_v1_2_module_media, docs_architecture_v1_2_module_contact, docs_architecture_v1_2_module_notifications, docs_architecture_v1_2_module_admin [EXTRACTED 1.00]
- **OTP Login/Registration Flow** — docs_architecture_v1_2_otp_passwordless_auth, docs_architecture_v1_2_registration_ticket, docs_architecture_v1_2_atomic_registration [EXTRACTED 1.00]
- **Owner Upgrade Request/Validation Workflow** — docs_architecture_v1_2_owner_request_flow, docs_architecture_v1_2_admin_bootstrap, docs_technical_spec_mvp_v1_2_owner_requests_table [EXTRACTED 1.00]
- **MyHouse Locked Decisions Enforced Across Project Instructions and Review Skills** — agents_rules_insrtruction_for_my_house_key_decisions_locked, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Shared Four-Phase Review Structure** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_code_review_backend_skill_four_phase_review_process, claude_skills_code_review_frontend_skill_four_phase_review_process [INFERRED 0.85]
- **Refresh Token Handling Invariants Across Review Skills** — claude_skills_code_review_backend_skill_refresh_token_rotation_invariant, claude_skills_code_review_frontend_skill_refresh_token_cookie_invariant, claude_skills_code_review_backend_skill_myhouse_invariants, claude_skills_code_review_frontend_skill_myhouse_invariants [INFERRED 0.85]
- **Skills Deferring to ARCHITECTURE.md / TECHNICAL_SPEC_MVP.md** — claude_skills_code_review_backend_skill_code_review_backend, claude_skills_code_review_frontend_skill_code_review_frontend, claude_skills_github_ticket_skill_github_ticket, docs_architecture_doc, docs_technical_spec_mvp_doc [INFERRED 0.85]
- **Extract-Build-Cluster pipeline flow (Steps 3-5)** — claude_skills_graphify_skill_step3_extract_entities, claude_skills_graphify_skill_part_a_structural_extraction, claude_skills_graphify_skill_part_b_semantic_extraction, claude_skills_graphify_skill_part_c_merge_ast_semantic, claude_skills_graphify_skill_step4_build_cluster_analyze, claude_skills_graphify_skill_step5_label_communities [EXTRACTED 1.00]
- **query/path/explain feedback-loop family** — claude_skills_graphify_references_query_traversal, claude_skills_graphify_references_query_graphify_path, claude_skills_graphify_references_query_graphify_explain, claude_skills_graphify_references_query_save_result [EXTRACTED 1.00]
- **Incremental-update integrity guards (stamping, pruning, manifest)** — claude_skills_graphify_skill_stamped_manifest_files_2015, claude_skills_graphify_references_update_replace_on_reextract_1344, claude_skills_graphify_references_update_incremental_update, claude_skills_graphify_skill_step9_manifest_cost_cleanup [INFERRED 0.85]
- **Backend CI Quality Gate (fmt, clippy, deny, audit, build, coverage)** — github_workflows_ci_backend_yml_backend_fmt, github_workflows_ci_backend_yml_backend_clippy, github_workflows_ci_backend_yml_backend_deny, github_workflows_ci_backend_yml_backend_audit, github_workflows_ci_backend_yml_backend_build, github_workflows_ci_backend_yml_backend_coverage [EXTRACTED 1.00]
- **OpenAPI-to-TypeScript Generation Chain (utoipa → gen_openapi → types.ts → codegen job)** — github_workflows_ci_backend_yml_gen_openapi_bin [EXTRACTED 1.00]
- **Owner Request Approval Notification Flow** — backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.85]
- **Notifications Module Email Template Set** — backend_src_modules_notifications_templates_otp, backend_src_modules_notifications_templates_welcome, backend_src_modules_notifications_templates_owner_request_approved, backend_src_modules_notifications_templates_owner_request_received, backend_src_modules_notifications_templates_owner_request_rejected [INFERRED 0.80]
- **Dev Environment Docker Compose Stack** — backend_compose_backend_backend_dev, frontend_compose_frontend_frontend_dev, docker_compose_db, docker_compose_mailhog [INFERRED 0.85]

## Communities (158 total, 44 thin omitted)

### Community 0 - ".new"
Cohesion: 0.05
Nodes (66): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+58 more)

### Community 1 - "users/service.rs"
Cohesion: 0.09
Nodes (43): UpdateMeDto, a_missing_or_failing_key_does_not_abort_the_remaining_keys(), bootstrap_admin(), delete_account(), delete_previous_avatar(), delete_storage_objects(), deletes_every_enumerated_key(), deletes_the_key_behind_the_previous_avatar_url() (+35 more)

### Community 2 - "AppCacheProvider"
Cohesion: 0.10
Nodes (43): AppCache, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache(), Arc, AtomicU32 (+35 more)

### Community 3 - "components/index.ts"
Cohesion: 0.08
Nodes (41): AlertProps, AlertVariant, variantConfig, ButtonProps, ButtonSize, ButtonVariant, sizeClasses, variantClasses (+33 more)

### Community 4 - "config/mod.rs"
Cohesion: 0.09
Nodes (50): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, empty_trusted_proxies_trusts_nothing(), loads_admin_bootstrap_email_when_enabled(), loads_valid_config() (+42 more)

### Community 5 - "admin/api.ts"
Cohesion: 0.20
Nodes (17): AdminOwnerRequest, AdminOwnerRequestDetail, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocument, getOwnerRequest(), getOwnerRequestDocument(), listOwnerRequests(), ListOwnerRequestsParams (+9 more)

### Community 6 - "owner_requests/dto.rs"
Cohesion: 0.09
Nodes (47): admin_detail_dto_falls_back_to_empty_documents_on_malformed_json(), admin_detail_dto_never_exposes_storage_key(), admin_detail_row(), admin_list_dto_omits_identity_data_and_documents(), admin_row(), AdminOwnerRequestDetailDto, AdminOwnerRequestDetailResponse, AdminOwnerRequestDocumentDto (+39 more)

### Community 7 - "jwt.rs"
Cohesion: 0.08
Nodes (37): Algorithm, NewAccount, RefreshTokenLookup, Option, Uuid, hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest() (+29 more)

### Community 8 - "rate_limit.rs"
Cohesion: 0.14
Nodes (28): allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it(), distinct_clients_get_distinct_counters(), falls_back_to_peer_ip_when_trusted_header_is_missing(), headers_with_xff(), ignores_x_forwarded_for_from_an_untrusted_peer(), middleware_passes_then_rejects_with_429_and_retry_after(), peer(), rate_limit() (+20 more)

### Community 9 - "owner_requests/service.rs"
Cohesion: 0.05
Nodes (54): a_doc_id_from_a_different_request_does_not_resolve(), a_missing_storage_object_surfaces_as_document_not_found(), a_pdf_renamed_into_an_image_slot_is_a_shape_violation(), accepts_one_pdf_with_no_side(), accepts_two_images_as_front_and_back(), as_document_not_found(), as_shape_error(), ClassifiedDocument (+46 more)

### Community 10 - "UserRow"
Cohesion: 0.11
Nodes (30): AvatarUploadForm, response_envelope_serializes_the_profile_fields(), row(), row_maps_to_dto_field_for_field(), From, Option, Self, String (+22 more)

### Community 11 - "Mailer"
Cohesion: 0.08
Nodes (36): Address, AddressError, AsyncSmtpTransport, builds_successfully_with_valid_config(), Mailer, MailerError, rejects_malformed_smtp_from(), Formatter (+28 more)

### Community 12 - "MyHouse Project Instructions (Agents)"
Cohesion: 0.06
Nodes (36): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+28 more)

### Community 13 - "ProfileForm.tsx"
Cohesion: 0.05
Nodes (62): RootLayout(), RequireAdmin(), RequireAdminProps, getOwnerRequestStatus(), OwnerRequest, OwnerRequestResponse, ownerRequestStatusQueryKey, OwnerRequestStatusResponse (+54 more)

### Community 14 - "local_fs.rs"
Cohesion: 0.17
Nodes (23): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_on_missing_key_returns_not_found_not_generic_storage_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes(), Bytes (+15 more)

### Community 15 - "ListingFeed.tsx"
Cohesion: 0.19
Nodes (19): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+11 more)

### Community 16 - "OtpVerifyForm.tsx"
Cohesion: 0.15
Nodes (17): AuthFlow(), AuthStepIndicator(), markInterrupted(), readInterrupted(), Screen, STEP_ORDER, OtpCodeInput(), OtpCodeInputProps (+9 more)

### Community 17 - "auth/index.ts"
Cohesion: 0.17
Nodes (23): logout(), OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken, RefreshResponse, refreshSession(), registerAccount(), RegisterPayload (+15 more)

### Community 18 - "compilerOptions"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 19 - "extractors.rs"
Cohesion: 0.15
Nodes (17): AppJson, AppJson<T>, AuthState, bearer_token(), resolve_identity(), Arc, PgPool, Request (+9 more)

### Community 20 - "Code Review — Backend Skill"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 21 - "AppError"
Cohesion: 0.25
Nodes (19): create_account(), db_err(), email_exists(), find_by_hash(), find_user_by_email(), insert_refresh_token(), revoke(), revoke_all_for_user() (+11 more)

### Community 22 - "devDependencies"
Cohesion: 0.10
Nodes (21): eslint, @eslint/js, eslint-plugin-react-refresh, devDependencies, eslint, @eslint/js, eslint-plugin-react-refresh, openapi-typescript (+13 more)

### Community 23 - "SiteHeader.tsx"
Cohesion: 0.10
Nodes (12): BadgeProps, BadgeSize, sizeClasses, toneClasses, MobileMenu(), NavItem(), NavLinkSpec, OWNER_LINKS (+4 more)

### Community 24 - "OwnerRequestDetail.tsx"
Cohesion: 0.14
Nodes (16): DocumentViewer(), OwnerRequestDetail(), OwnerRequestQueueList(), STATUS_OPTIONS, OwnerRequestStatus, ownerRequestStatusBadge(), STATUS_BADGE, reviewErrorMessage() (+8 more)

### Community 25 - "compilerOptions"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 26 - "auth/handler.rs"
Cohesion: 0.25
Nodes (17): logout(), otp_request(), otp_verify(), refresh(), register(), CookieJar, Json, Result (+9 more)

### Community 27 - "client.ts"
Cohesion: 0.23
Nodes (14): AccessTokenGetter, apiDelete(), apiPatch(), apiPut(), ErrorEnvelope, isAuthPath(), jsonInit(), QueryValue (+6 more)

### Community 28 - "TokenDecoder"
Cohesion: 0.21
Nodes (8): Inner, Arc, PgPool, Self, StorageProvider, Send, Sync, TokenDecoder

### Community 29 - "admin/handler.rs"
Cohesion: 0.29
Nodes (14): AdminOwnerRequestDetailResponse, get_owner_request(), get_owner_request_document(), list_owner_requests(), review_owner_request(), IntoResponse, Json, Path (+6 more)

### Community 30 - "users/handler.rs"
Cohesion: 0.29
Nodes (14): delete_me(), get_me(), multipart_error(), read_file_field(), Bytes, CookieJar, Json, Multipart (+6 more)

### Community 31 - ".mcp.json"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 32 - "AppState"
Cohesion: 0.28
Nodes (14): ApiDoc, AppState, admin_router(), avatar_router(), build_router(), merged_router(), openapi_spec(), owner_request_router() (+6 more)

### Community 33 - "router.tsx"
Cohesion: 0.13
Nodes (13): AdminLayout(), AuthLayout(), RequireAuth(), RequireAuthProps, AuthFlow, ListingDetail, ListingFeed, OwnerRequestDetail (+5 more)

### Community 34 - "owner_requests/repository.rs"
Cohesion: 0.28
Nodes (17): count_for_admin(), db_err(), find_by_id_for_admin(), find_current_for_user(), insert_pending(), list_for_admin(), pending_exists_for_user(), review_for_admin() (+9 more)

### Community 35 - "DeleteAccountSection.tsx"
Cohesion: 0.11
Nodes (17): REASONS, ReportListingModal(), ReportListingModalProps, DeleteAccountSection(), DeleteAccountSectionProps, Modal(), ModalProps, ModalSize (+9 more)

### Community 36 - "errors.rs"
Cohesion: 0.23
Nodes (13): ErrorBody, ErrorEnvelope, parse_envelope(), Response, StatusCode, String, Value, test_bad_request_carries_detail() (+5 more)

### Community 37 - "dependencies"
Cohesion: 0.13
Nodes (15): clsx, dependencies, clsx, lucide-react, react, react-dom, react-hook-form, react-router (+7 more)

### Community 38 - "Module: users"
Cohesion: 0.18
Nodes (15): Single Admin Account Bootstrap, Module: admin, Module: auth, Module: contact, Module: listings, Module: media, Module: notifications, Module: search (+7 more)

### Community 39 - "Design Handoff README"
Cohesion: 0.15
Nodes (15): MyHouse Design Tokens (Plan & Brass), Dimension Rule Signature Device, MH-17 Superseded Cream/Terracotta Palette, Design Handoff Repo Sync Log & Screen Map, My House - Plan & Laiton Design Mockup, My House - Screens Design Mockup, Design Handoff README, Admin Approve as Sole Solid Semantic Button (+7 more)

### Community 40 - "auth/dto.rs"
Cohesion: 0.26
Nodes (13): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+5 more)

### Community 41 - "Extraction subagent prompt (full)"
Cohesion: 0.18
Nodes (13): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part A - Structural extraction for code files (AST), Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction (+5 more)

### Community 42 - "admin identity-document read exception"
Cohesion: 0.15
Nodes (14): admin identity-document read exception, ARCHITECTURE.md (referenced doc), backend/Dockerfile, docker-compose.yml, backend/.env.example variables, frontend/Dockerfile, GET /admin/owner-requests/:id/documents/:doc_id, infra/mailer.rs (lettre SMTP client) (+6 more)

### Community 43 - "UnimplementedStorage"
Cohesion: 0.26
Nodes (8): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage

### Community 44 - "AuthUser"
Cohesion: 0.23
Nodes (14): get_owner_request_status(), multipart_error(), read_submission(), Json, Multipart, MultipartError, Result, State (+6 more)

### Community 45 - "scripts"
Cohesion: 0.15
Nodes (13): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+5 more)

### Community 46 - "MH-49-BE / MH-49-FE — Admin approve/reject decision on owner requests"
Cohesion: 0.22
Nodes (8): Backend design, Comments, Context, MH-49-BE — actual acceptance criteria (issue #111), MH-49-BE / MH-49-FE — Admin approve/reject decision on owner requests, MH-49-FE (separate branch, not built this session) — actual AC (issue #112), Testing, Verification

### Community 47 - "StorageProvider Trait"
Cohesion: 0.17
Nodes (12): Account Deletion Cascade + Storage Cleanup, AwsS3Storage (V2), LocalFsStorage, R-06: Filesystem storage not shared across instances, StorageProvider Trait, automatic cover photo selection, AwsS3Storage (V2), listing_media table (+4 more)

### Community 48 - "resolve_request_id"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), HeaderMap, Next, Request, Response (+2 more)

### Community 49 - "Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 50 - "CI Backend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 51 - ".run"
Cohesion: 0.27
Nodes (7): AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), Box

### Community 52 - "health.rs"
Cohesion: 0.29
Nodes (7): check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus

### Community 53 - "sqlx::query! / query_as! compile-time checked queries"
Cohesion: 0.22
Nodes (10): DATABASE_URL compile-time requirement, fn_set_updated_at trigger, Partial unique indexes for 'at most one active X' invariants, sqlx::query! / query_as! compile-time checked queries, sqlx QueryBuilder, Repository tests in rolled-back transaction (integration feature flag), Schema conventions (UUID PK, timestamps, enums), No string-interpolated SQL rule (+2 more)

### Community 54 - "save-result feedback loop"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 55 - "users table"
Cohesion: 0.24
Nodes (10): fn_cascade_owner_name_to_listings trigger fn, fn_update_listing_search_vector trigger fn, listing_status enum, listing_type enum, listings module, listings table, search module, user_role enum (+2 more)

### Community 56 - "mh-15-owner-request.html wireframe"
Cohesion: 0.20
Nodes (10): mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire'), Identity document dropzone component, Request approved status page (+2 more)

### Community 57 - "backend-dev service"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 58 - "MokaStore"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 59 - "/graphify add <url>"
Cohesion: 0.29
Nodes (6): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), /graphify add <url> (Codex), --watch flag (Codex)

### Community 60 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 61 - "/graphify command"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 62 - "shared module"
Cohesion: 0.29
Nodes (8): AuthUser Extractor, Role Model (seeker/owner/admin), AppError, frontend features/ directory, OpenAPI → TypeScript type generation pipeline, PaginatedResponse<T>, RBAC role guards, shared module

### Community 63 - "package.json"
Cohesion: 0.25
Nodes (7): name, typescript, overrides, openapi-typescript, private, type, version

### Community 64 - "Pagination.tsx"
Cohesion: 0.36
Nodes (4): Pagination(), PaginationProps, getPageItems(), PageItem

### Community 65 - "CI Job: Frontend OpenAPI Codegen"
Cohesion: 0.43
Nodes (8): CI Job: Frontend Build, CI Job: Frontend OpenAPI Codegen, CI Job: Frontend Docker Build, CI Job: Frontend Install, CI Job: Frontend Lint, CI Job: Frontend Prettier, CI Job: Frontend Unit Tests, CI Job: Frontend TypeScript

### Community 66 - "CLAUDE.md — MyHouse project instructions"
Cohesion: 0.29
Nodes (7): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), mcp/.toolbox/tool.yaml — postgres-local MCP toolbox source

### Community 67 - "MokaStore<K, V>"
Cohesion: 0.52
Nodes (4): MokaStore<K, V>, K, Option, V

### Community 68 - "ON DELETE CASCADE — relational-only cleanup rule"
Cohesion: 0.29
Nodes (6): GET /admin/owner-requests/:id/documents/:doc_id, ARCHITECTURE.md, ON DELETE CASCADE — relational-only cleanup rule, listings.price missing index (known gap R-07), listings/search index checklist (idx_listings_*), Sensitive columns exclusion (identity_data, identity_documents JSONB)

### Community 69 - "Multiple repos cross-repo graph merge"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 70 - "Step 4 - Build graph, cluster, analyze, generate outputs"
Cohesion: 0.40
Nodes (5): Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML

### Community 71 - "users module"
Cohesion: 0.38
Nodes (7): admin module, media module, Notification stack (§3bis), notifications::service::send_xxx_email, synchronous best-effort email send, Testing strategy (§6), users module

### Community 72 - "Auth API endpoints (§4.1)"
Cohesion: 0.29
Nodes (7): Auth API endpoints (§4.1), auth module, OTP passwordless auth, owner request atomic submission, refresh token rotation, refresh_tokens table, registration_ticket

### Community 73 - "providers.tsx"
Cohesion: 0.32
Nodes (5): App(), Providers(), ProvidersProps, queryClient, router

### Community 74 - "Embedded React/TypeScript Rules Content"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 75 - "--update (incremental re-extraction)"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 76 - "connect_db"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 77 - "Step 9 - Save manifest, update cost tracker, clean up, and report"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 78 - "owner_requests table"
Cohesion: 0.40
Nodes (5): owner_request_status enum, owner_requests table, OwnerRequestForm.tsx, PATCH /admin/owner-requests/:id, POST /owner-requests

### Community 79 - "generate_otp_code"
Cohesion: 0.67
Nodes (3): generate_otp_code(), generated_code_is_always_six_digits(), String

### Community 80 - "sqlx"
Cohesion: 0.50
Nodes (4): Forward-only migrations policy, No ORM — Prisma/Supabase rejected, PostgreSQL, sqlx

### Community 81 - "git commit hook (graphify hook install)"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 82 - "mh-12-auth-flow.html wireframe"
Cohesion: 0.50
Nodes (4): mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 83 - "analyze job (rust + javascript-typescript matrix)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 84 - "Root Docker Compose"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 85 - "Owner Request Approved Email Template"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 87 - "Registration Ticket (opaque UUID)"
Cohesion: 0.67
Nodes (3): Atomic Registration via POST /auth/register, OTP Passwordless Authentication, Registration Ticket (opaque UUID)

### Community 88 - "mh-13-feed-detail.html wireframe"
Cohesion: 0.67
Nodes (3): mh-13-feed-detail.html wireframe, Public feed page (grid of cover photos), Listing detail page

### Community 90 - "README.md — Project Overview and Setup"
Cohesion: 0.67
Nodes (3): README.md — Project Overview and Setup, Conventional Commits convention, Trunk-based development branching strategy

### Community 105 - "Extraction subagent prompt (compact)"
Cohesion: 0.50
Nodes (5): Compact prompt rationale (return inline, no CHUNK_PATH), Extraction subagent prompt (compact), spawn_agent / wait_agent / close_agent mechanism, Step B2 - Dispatch ALL subagents (Codex spawn_agent), multi_agent feature flag (~/.codex/config.toml)

### Community 106 - "useReviewOwnerRequest.ts"
Cohesion: 0.60
Nodes (4): ReviewDecision, reviewOwnerRequest(), ReviewOwnerRequestInput, useReviewOwnerRequest()

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `README Writing Rules Skill` → `react-typecrypt.md Rules File`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with
- `README Writing Rules Skill` → `Embedded React/TypeScript Rules Content`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with

## Knowledge Gaps
- **312 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+307 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **44 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `react-typecrypt.md Rules File`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `Embedded React/TypeScript Rules Content`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **Why does `AppError` connect `AppError` to `.new`, `users/service.rs`, `AppCacheProvider`, `owner_requests/repository.rs`, `errors.rs`, `jwt.rs`, `owner_requests/service.rs`, `UserRow`, `UnimplementedStorage`, `Mailer`, `AuthUser`, `local_fs.rs`, `extractors.rs`, `auth/handler.rs`, `admin/handler.rs`, `users/handler.rs`?**
  _High betweenness centrality (0.165) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `config/mod.rs` to `AppState`, `users/service.rs`, `Mailer`, `TokenDecoder`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState` to `.new`, `AuthUser`, `.run`, `health.rs`, `extractors.rs`, `auth/handler.rs`, `TokenDecoder`, `admin/handler.rs`, `users/handler.rs`?**
  _High betweenness centrality (0.027) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _312 weakly-connected nodes found - possible documentation gaps or missing edges._