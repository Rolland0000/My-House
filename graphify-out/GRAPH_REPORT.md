# Graph Report - .  (2026-09-12)

## Corpus Check
- 17 files · ~130,198 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1645 nodes · 3100 edges · 180 communities (119 shown, 61 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 91 edges (avg confidence: 0.84)
- Token cost: 113,998 input · 0 output

## Community Hubs (Navigation)
- Owner Request API Client
- User Profile DTOs & Validation
- Listing Detail DTOs
- Storage Provider Abstraction
- App Cache (moka)
- App Config from Env
- Owner Request HTTP Handler
- Auth Domain Model
- Alert & Badge Components
- Database Rules Doc
- App Shell & Query Providers
- Auth Flow Wizard (Frontend)
- IP Rate Limiter
- Local Filesystem Storage
- Listings API Client
- Owner Request Document Classification
- Frontend App tsconfig
- Code Review Backend Skill
- Architecture Doc: Auth & Storage ADRs
- Frontend ESLint Deps
- Frontend Node tsconfig
- Mailer & SMTP Config
- Auth HTTP Handlers
- Notifications Service & Templates
- Root Layout & Site Header
- AppState Bootstrap
- Technical Spec: Owner Request Deviations
- MCP Server Config
- Auth Layout & Router
- OpenAPI Doc & Route Wiring
- Profile Field Validation
- Frontend UI Deps (clsx, lucide-react)
- Design Tokens & Handoff Docs
- OTP Auth DTOs
- Listing Report Modal
- Owner Request DTOs
- Graphify Extraction Spec Doc
- Frontend Package Scripts
- RequireAuth & Button Component
- Owner Request Model Types
- File Magic-Byte Validation
- Request ID Middleware
- Graphify Export Formats Doc
- CI Backend Workflow
- App Server Bootstrap
- Health Check Endpoint
- Graphify Query Doc
- Owner/Listing Management Wireframe
- Docker Compose Services & OTP Template
- Moka Cache Store
- Docker Rules Skill
- Graphify Transcription Doc
- Frontend Package Metadata
- Pagination Component & Utility
- CI Frontend Workflow
- Project CLAUDE.md Instructions
- Cache Provider Trait
- Graphify Add/Watch Doc
- Graphify Multi-Repo Merge Doc
- Frontend React/TS Rules
- Graphify Update/Cluster-Only Doc
- DB Connection Pool
- Graphify Honesty Rules
- Graphify Build/Cluster Steps Doc
- Codex Graphify Variant
- OTP Code Generation
- Graphify Hooks Doc
- Auth Flow Wireframe
- CodeQL & Gitleaks Workflows
- Docker Compose Files
- Owner Request Email Templates
- Pre-Tool-Use Hook Script
- Architecture Doc: Atomic Registration
- Feed/Detail Wireframe
- Frontend Root tsconfig
- README Project Overview
- OpenAPI Codegen Binary
- Architecture Doc: Auth Extractor & Roles
- Architecture Doc: No-Proxy Media Rule
- Architecture Doc: Search Vector Trigger
- Architecture Doc: Refresh Token Cookie
- Frontend Hooks ESLint Dep
- Frontend Globals Dep
- Tailwind CSS Dep
- React DOM Types Dep
- typescript-eslint Dep
- Vite Dep
- Admin Layout (Frontend)
- Pre-Commit Hook Script
- Infra Module Root
- Backend Lib Root
- Middleware Module Root
- Admin Module Root
- Auth Module Root
- Contact Module Root
- Media Module Root
- Modules Root
- Notifications Module Root
- Owner Requests Module Root
- Search Module Root
- Users Module Root
- Duration Type
- Response Type
- Arc Type
- Request Type
- Shared Module Root
- Technical Spec MVP Doc Root
- Villa Night Exterior Photo (OIP 10)
- Bedroom Reference Photo (OIP 11)
- Green Bedroom Photo (OIP 1)
- Ocean-View Bedroom Photo (OIP 2)
- Bedroom Reference Photo (OIP 4)
- Bedroom with Garden Deck Photo (OIP 5)
- Villa with Pool Photo (OIP 6)
- Bedroom Interior Photo (OIP 9)
- Bedroom Reference Photo (OIP)
- Frontend ESLint Config
- Admin API Client
- Admin Feature Index
- Contact Feature Index
- Search API Client
- Search Hook
- Search Feature Index
- Vite Env Types
- Vite Config
- Multipart Type
- MultipartError Type
- OtpRequestResponse Type
- OtpVerifyResponse Type
- OwnerRequestId Type
- RefreshResponse Type
- RegisterResponse Type
- Value Type
- Generic Value Reference
- Notifications Module Spec
- Search Endpoint Spec
- Users Endpoints Spec
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- MyHouse Photo Asset
- OTP Request Response Type
- OTP Verify Response Type
- Refresh Response Type
- Register Response Type
- Generic Value Reference

## God Nodes (most connected - your core abstractions)
1. `AppError` - 119 edges
2. `AppCacheProvider` - 26 edges
3. `cn()` - 23 edges
4. `AppState` - 20 edges
5. `set_valid_env()` - 20 edges
6. `AppCache` - 19 edges
7. `AppConfig` - 18 edges
8. `compilerOptions` - 18 edges
9. `UserRow` - 17 edges
10. `ConfigError` - 16 edges

## Surprising Connections (you probably didn't know these)
- `--update (incremental re-extraction)` --semantically_similar_to--> `--update (incremental re-extraction) (Codex)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/references/update.md → .codex/skills/graphify/references/update.md
- `AGENTS.md — graphify trigger instructions` --semantically_similar_to--> `CLAUDE.md — MyHouse project instructions`  [INFERRED] [semantically similar]
  AGENTS.md → CLAUDE.md
- `/graphify command` --semantically_similar_to--> `/graphify command (Codex variant)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md
- `Step B2 - Dispatch ALL subagents in a single message` --semantically_similar_to--> `Step B2 - Dispatch ALL subagents (Codex spawn_agent)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md
- `Step 9 - Save manifest, update cost tracker, clean up, and report` --semantically_similar_to--> `Step 9 - Save manifest, update cost tracker, clean up (Codex)`  [INFERRED] [semantically similar]
  .claude/skills/graphify/SKILL.md → .codex/skills/graphify/SKILL.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
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
- **Duplicated Rule Sets Between .agents/rules and .claude/rules** — agents_rules_database_rules_myhouse_db_rules, claude_rules_database_myhouse_db_rules, agents_rules_rust_general_rules_myhouse_rust_rules [INFERRED 0.85]
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

## Communities (180 total, 61 thin omitted)

### Community 0 - "Owner Request API Client"
Cohesion: 0.05
Nodes (66): ListingDetailDto, ListingDetailResponse, ListingMediaDto, ListingSummaryDto, ListListingsQuery, OwnerDetailDto, OwnerSummaryDto, From (+58 more)

### Community 1 - "User Profile DTOs & Validation"
Cohesion: 0.10
Nodes (43): AppCache, build_auth_challenge_cache(), build_cache_provider(), build_ip_rate_limit_cache(), build_otp_rate_limit_cache(), build_refresh_replay_cache(), Arc, AtomicU32 (+35 more)

### Community 2 - "Listing Detail DTOs"
Cohesion: 0.09
Nodes (50): admin_bootstrap_defaults_to_disabled_when_absent(), app_port_defaults_to_3000_when_absent(), AppConfig, AppEnv, ConfigError, empty_trusted_proxies_trusts_nothing(), loads_admin_bootstrap_email_when_enabled(), loads_valid_config() (+42 more)

### Community 3 - "Storage Provider Abstraction"
Cohesion: 0.08
Nodes (42): logout(), OtpRequestResponse, OtpVerifyResponse, OtpVerifyToken, RefreshResponse, refreshSession(), registerAccount(), RegisterPayload (+34 more)

### Community 4 - "App Cache (moka)"
Cohesion: 0.07
Nodes (46): OwnerRequestDto, OwnerRequestResponse, OwnerRequestStatusResponse, OwnerRequestSubmissionForm, row(), From, Option, Self (+38 more)

### Community 5 - "App Config from Env"
Cohesion: 0.08
Nodes (37): Algorithm, NewAccount, RefreshTokenLookup, Option, Uuid, hash_otp_code(), hash_refresh_token(), hashes_deterministically_to_a_64_char_hex_digest() (+29 more)

### Community 6 - "Owner Request HTTP Handler"
Cohesion: 0.10
Nodes (38): AppCacheProvider, Arc, delete_me(), get_me(), multipart_error(), read_file_field(), AppState, Bytes (+30 more)

### Community 7 - "Auth Domain Model"
Cohesion: 0.05
Nodes (44): Cascade and Filesystem Cleanup Ordering, Migration Conventions, MyHouse Database Rules (sqlx/PostgreSQL), Listings/Search Index Performance Rules, sqlx Query Conventions (query!/query_as!), Schema Conventions (UUID PK, timestamps, enums, partial unique indexes), SQL Injection Prevention / Sensitive Column Exclusion, Repository Test Transaction Rollback Pattern (+36 more)

### Community 8 - "Alert & Badge Components"
Cohesion: 0.15
Nodes (24): getListing(), ListingDetail, ListingStatus, ListingSummary, ListingType, listListings(), ListListingsParams, ListListingsResult (+16 more)

### Community 9 - "Database Rules Doc"
Cohesion: 0.14
Nodes (28): allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it(), distinct_clients_get_distinct_counters(), falls_back_to_peer_ip_when_trusted_header_is_missing(), headers_with_xff(), ignores_x_forwarded_for_from_an_untrusted_peer(), middleware_passes_then_rejects_with_429_and_retry_after(), peer(), rate_limit() (+20 more)

### Community 10 - "App Shell & Query Providers"
Cohesion: 0.17
Nodes (22): delete_on_missing_key_returns_typed_error_not_panic(), delete_removes_existing_file(), LocalFsStorage, presigned_url_returns_not_implemented_error(), read_rejects_key_with_parent_dir_component(), read_returns_previously_uploaded_bytes(), Bytes, Duration (+14 more)

### Community 11 - "Auth Flow Wizard (Frontend)"
Cohesion: 0.12
Nodes (18): AvatarUploadForm, response_envelope_serializes_the_profile_fields(), row(), row_maps_to_dto_field_for_field(), From, Option, Role, Self (+10 more)

### Community 12 - "IP Rate Limiter"
Cohesion: 0.19
Nodes (15): getMe(), Profile, profileQueryKey, updateMe(), UpdateProfilePayload, uploadAvatar(), UserResponse, OwnerRequestStatusBlock() (+7 more)

### Community 13 - "Local Filesystem Storage"
Cohesion: 0.08
Nodes (23): compilerOptions, allowImportingTsExtensions, jsx, lib, module, moduleDetection, moduleResolution, noEmit (+15 more)

### Community 14 - "Listings API Client"
Cohesion: 0.11
Nodes (23): Backend Review Checklist (reference/checklist.md), AuthUser is_active Re-verification Invariant, Code Review — Backend Skill, Four-Phase Backend Review Process, MyHouse Backend Review Invariants, Refresh Token Rotation & Family Revocation Invariant, Backend Review Severity Labels, StorageProvider Abstraction Requirement (+15 more)

### Community 15 - "Owner Request Document Classification"
Cohesion: 0.23
Nodes (18): AppConfig, a_missing_or_failing_key_does_not_abort_the_remaining_keys(), bootstrap_admin(), delete_account(), delete_previous_avatar(), delete_storage_objects(), deletes_every_enumerated_key(), deletes_the_key_behind_the_previous_avatar_url() (+10 more)

### Community 16 - "Frontend App tsconfig"
Cohesion: 0.10
Nodes (21): eslint, @eslint/js, eslint-plugin-react-refresh, devDependencies, eslint, @eslint/js, eslint-plugin-react-refresh, openapi-typescript (+13 more)

### Community 17 - "Code Review Backend Skill"
Cohesion: 0.21
Nodes (13): getOwnerRequestStatus(), OwnerRequest, OwnerRequestResponse, ownerRequestStatusQueryKey, OwnerRequestStatusResponse, submitOwnerRequest(), OwnerRequestStatus(), useOwnerRequestStatus() (+5 more)

### Community 18 - "Architecture Doc: Auth & Storage ADRs"
Cohesion: 0.16
Nodes (15): AlertProps, AlertVariant, variantConfig, Input(), InputProps, Select(), SelectOption, SelectProps (+7 more)

### Community 19 - "Frontend ESLint Deps"
Cohesion: 0.10
Nodes (11): Badge(), BadgeProps, BadgeSize, BadgeTone, sizeClasses, toneClasses, NavLinkSpec, OWNER_LINKS (+3 more)

### Community 20 - "Frontend Node tsconfig"
Cohesion: 0.17
Nodes (16): assert_key_shape(), avatar_key(), avatar_key_from_url(), avatar_key_from_url_round_trips_a_generated_key(), avatar_key_has_expected_shape(), extension_is_preserved_verbatim(), generated_keys_are_unique_across_calls(), listing_media_key() (+8 more)

### Community 21 - "Mailer & SMTP Config"
Cohesion: 0.17
Nodes (13): FileWithPreview, FormValues, ID_TYPE_OPTIONS, ImageSlotProps, Modality, OwnerRequestForm(), PdfSlotProps, preCheckImage() (+5 more)

### Community 22 - "Auth HTTP Handlers"
Cohesion: 0.16
Nodes (17): deleteAccount(), DeleteAccountSection(), DeleteAccountSectionProps, useDeleteAccount(), AccessTokenGetter, apiDelete(), apiPut(), buildQueryString() (+9 more)

### Community 23 - "Notifications Service & Templates"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+11 more)

### Community 24 - "Root Layout & Site Header"
Cohesion: 0.16
Nodes (15): Address, AddressError, AsyncSmtpTransport, builds_successfully_with_valid_config(), Mailer, MailerError, rejects_malformed_smtp_from(), Formatter (+7 more)

### Community 25 - "AppState Bootstrap"
Cohesion: 0.27
Nodes (17): logout(), otp_request(), otp_verify(), refresh(), register(), AppState, CookieJar, Json (+9 more)

### Community 26 - "Technical Spec: Owner Request Deviations"
Cohesion: 0.18
Nodes (18): NotificationTemplate, OtpTemplate, OwnerRequestApprovedTemplate, OwnerRequestReceivedTemplate, OwnerRequestRejectedTemplate, render(), renders_otp_template_with_sample_context(), renders_owner_request_approved_template_with_sample_context() (+10 more)

### Community 27 - "MCP Server Config"
Cohesion: 0.24
Nodes (9): AppState, Inner, Arc, PgPool, Self, StorageProvider, Send, Sync (+1 more)

### Community 28 - "Auth Layout & Router"
Cohesion: 0.23
Nodes (15): db_err(), find_current_for_user(), insert_pending(), pending_exists_for_user(), Error, Option, PgPool, Result (+7 more)

### Community 29 - "OpenAPI Doc & Route Wiring"
Cohesion: 0.19
Nodes (11): update_me(), optional_name(), optional_phone(), repeat(), required_name(), required_name_trims_and_accepts_the_upper_bound(), required_phone(), required_phone_trims_and_accepts_the_upper_bound() (+3 more)

### Community 30 - "Profile Field Validation"
Cohesion: 0.15
Nodes (16): DATABASE_URI, GITHUB_PERSONAL_ACCESS_TOKEN, npx, uvx, context7, filesystem, git, github (+8 more)

### Community 31 - "Frontend UI Deps (clsx, lucide-react)"
Cohesion: 0.13
Nodes (13): RequireAuth(), RequireAuthProps, Button(), ButtonProps, ButtonSize, ButtonVariant, sizeClasses, variantClasses (+5 more)

### Community 32 - "Design Tokens & Handoff Docs"
Cohesion: 0.34
Nodes (14): ApiDoc, admin_router(), avatar_router(), build_router(), merged_router(), openapi_spec(), owner_request_router(), owner_router() (+6 more)

### Community 33 - "OTP Auth DTOs"
Cohesion: 0.37
Nodes (15): admin_exists(), delete_by_id(), find_by_id(), find_is_active(), list_listing_media_keys_for_owner(), list_owner_request_document_keys(), Option, PgPool (+7 more)

### Community 34 - "Listing Report Modal"
Cohesion: 0.15
Nodes (12): App(), Providers(), ProvidersProps, queryClient, router, ToastContext, ToastContextValue, ToastItem (+4 more)

### Community 35 - "Owner Request DTOs"
Cohesion: 0.15
Nodes (12): AuthLayout(), RootLayout(), AuthFlow, ListingDetail, ListingFeed, OwnerRequestForm, OwnerRequestStatus, ProfileForm (+4 more)

### Community 36 - "Graphify Extraction Spec Doc"
Cohesion: 0.25
Nodes (14): AppState, AuthUser, get_owner_request_status(), multipart_error(), read_submission(), Result, StatusCode, submit_owner_request() (+6 more)

### Community 37 - "Frontend Package Scripts"
Cohesion: 0.35
Nodes (14): create_account(), db_err(), email_exists(), find_by_hash(), find_user_by_email(), insert_refresh_token(), revoke(), revoke_all_for_user() (+6 more)

### Community 38 - "RequireAuth & Button Component"
Cohesion: 0.24
Nodes (12): ErrorBody, ErrorEnvelope, parse_envelope(), StatusCode, String, Value, test_bad_request_carries_detail(), test_internal_error_is_500() (+4 more)

### Community 39 - "Owner Request Model Types"
Cohesion: 0.13
Nodes (15): clsx, dependencies, clsx, lucide-react, react, react-dom, react-hook-form, react-router (+7 more)

### Community 40 - "File Magic-Byte Validation"
Cohesion: 0.15
Nodes (15): MyHouse Design Tokens (Plan & Brass), Dimension Rule Signature Device, MH-17 Superseded Cream/Terracotta Palette, Design Handoff Repo Sync Log & Screen Map, My House - Plan & Laiton Design Mockup, My House - Screens Design Mockup, Design Handoff README, Admin Approve as Sole Solid Semantic Button (+7 more)

### Community 41 - "Request ID Middleware"
Cohesion: 0.19
Nodes (9): DimensionRuleProps, FileDropzone(), FileDropzoneProps, FormField(), FormFieldChildProps, FormFieldProps, TextArea(), TextAreaProps (+1 more)

### Community 42 - "Graphify Export Formats Doc"
Cohesion: 0.26
Nodes (13): OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse, OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto (+5 more)

### Community 43 - "CI Backend Workflow"
Cohesion: 0.18
Nodes (14): EXTRACTED/INFERRED/AMBIGUOUS confidence rubric, Extraction JSON schema, Node ID format rule ({stem}_{entity}), source_file verbatim rule, Extraction subagent prompt (full), Part B - Semantic extraction (parallel subagents), Step B0 - Check extraction cache first, Step B1 - Split into chunks (+6 more)

### Community 44 - "App Server Bootstrap"
Cohesion: 0.15
Nodes (14): admin identity-document read exception, ARCHITECTURE.md (referenced doc), backend/Dockerfile, docker-compose.yml, backend/.env.example variables, frontend/Dockerfile, GET /admin/owner-requests/:id/documents/:doc_id, infra/mailer.rs (lettre SMTP client) (+6 more)

### Community 45 - "Health Check Endpoint"
Cohesion: 0.26
Nodes (8): Bytes, Duration, Result, Send, String, Sync, StorageProvider, UnimplementedStorage

### Community 46 - "Graphify Query Doc"
Cohesion: 0.15
Nodes (13): scripts, build, dev, format, generate:types, generate:types:ci, lint, predev (+5 more)

### Community 47 - "Owner/Listing Management Wireframe"
Cohesion: 0.30
Nodes (8): RecordingStorage, replace_avatar(), Bytes, Result, String, Vec, Duration, Mutex

### Community 48 - "Docker Compose Services & OTP Template"
Cohesion: 0.24
Nodes (6): accepts_pdf(), accepts_supported_image_formats(), Result, validate_image(), validate_pdf(), ValidatedFile

### Community 49 - "Moka Cache Store"
Cohesion: 0.17
Nodes (12): Account Deletion Cascade + Storage Cleanup, AwsS3Storage (V2), LocalFsStorage, R-06: Filesystem storage not shared across instances, StorageProvider Trait, automatic cover photo selection, AwsS3Storage (V2), listing_media table (+4 more)

### Community 50 - "Docker Rules Skill"
Cohesion: 0.25
Nodes (10): generates_a_valid_uuid_when_header_is_absent(), propagates_incoming_x_request_id_header_verbatim(), request_id(), resolve_request_id(), HeaderMap, Next, Request, Response (+2 more)

### Community 51 - "Graphify Transcription Doc"
Cohesion: 0.18
Nodes (11): Step 6b - Wiki export, Step 7 - Neo4j export, Step 7a - FalkorDB export, Step 7b - SVG export, Step 7c - GraphML export, Step 7d - MCP server (graphify.serve), Step 8 - Token reduction benchmark, Steps 6b-8 - Wiki, Neo4j, FalkorDB, SVG, GraphML, MCP, benchmark (+3 more)

### Community 52 - "Frontend Package Metadata"
Cohesion: 0.27
Nodes (11): Module: admin, Module: auth, Module: contact, Module: listings, Module: media, Module: notifications, Module: search, Module: shared (+3 more)

### Community 53 - "Pagination Component & Utility"
Cohesion: 0.42
Nodes (6): preCheck(), serverMessage(), AvatarUpload(), AvatarUploadProps, ACCEPTED_AVATAR_TYPES, isDisplayableMediaUrl()

### Community 54 - "CI Frontend Workflow"
Cohesion: 0.27
Nodes (11): CI Backend Workflow, backend_audit job (cargo-audit), backend_build job, backend_clippy job, backend_coverage job (cargo-llvm-cov + nextest), backend_deny job (cargo-deny), backend_doc job (cargo doc), backend_docker job (+3 more)

### Community 55 - "Project CLAUDE.md Instructions"
Cohesion: 0.27
Nodes (7): AppServer, Error, Result, Self, SocketAddr, shutdown_signal(), Box

### Community 56 - "Cache Provider Trait"
Cohesion: 0.29
Nodes (7): check(), check_storage(), HealthStatus, Json, State, StatusCode, StorageStatus

### Community 57 - "Graphify Add/Watch Doc"
Cohesion: 0.24
Nodes (10): /graphify explain, /graphify path, Step 0 - Constrained query expansion, save-result feedback loop, Step 1 - Traversal (BFS/DFS), Work memory outcome tags (useful/dead_end/corrected), For /graphify query (section pointer), /graphify explain (Codex) (+2 more)

### Community 58 - "Graphify Multi-Repo Merge Doc"
Cohesion: 0.24
Nodes (10): fn_cascade_owner_name_to_listings trigger fn, fn_update_listing_search_vector trigger fn, listing_status enum, listing_type enum, listings module, listings table, search module, user_role enum (+2 more)

### Community 59 - "Frontend React/TS Rules"
Cohesion: 0.20
Nodes (10): mh-14-listing-management.html wireframe, Owner's own listings ('Mes biens') page, Photo dropzone component, Publish/edit listing form page, mh-15-owner-request.html wireframe, Owner request form page ('Devenir propriétaire'), Identity document dropzone component, Request approved status page (+2 more)

### Community 60 - "Graphify Update/Cluster-Only Doc"
Cohesion: 0.22
Nodes (8): REASONS, ReportListingModal(), ReportListingModalProps, Modal(), ModalProps, ModalSize, sizeClasses, useToast()

### Community 61 - "DB Connection Pool"
Cohesion: 0.25
Nodes (9): backend-dev service, backend-prod service, OTP Login Code Email Template, Welcome Email Template, adminer service, db service (postgres:16-alpine), mailhog service (dev SMTP catcher), frontend-dev service (+1 more)

### Community 62 - "Graphify Honesty Rules"
Cohesion: 0.25
Nodes (7): MokaStore, MokaStore<K, V>, Duration, K, Self, V, Cache

### Community 63 - "Graphify Build/Cluster Steps Doc"
Cohesion: 0.22
Nodes (8): Debounce mechanism (default 3s), /graphify add <url>, --watch flag, For /graphify add and --watch (section pointer), Part A - Structural extraction for code files (AST), Step 3 - Extract entities and relationships, /graphify add <url> (Codex), --watch flag (Codex)

### Community 64 - "Codex Graphify Variant"
Cohesion: 0.25
Nodes (8): Docker Rules Skill, Dockerfile Best Practices, dockerignore Rules, Docker Forbidden Practices, Docker Logging Rules, Docker Networking Rules, Docker Security Rules, Docker Volumes Rules

### Community 65 - "OTP Code Generation"
Cohesion: 0.29
Nodes (8): Step 2.5 - Transcribe video/audio files, Whisper domain-hint prompt strategy, /graphify command, Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio detection, Step 2 - Detect files, Step 2.5 - Transcribe video/audio files (Codex), /graphify command (Codex variant)

### Community 66 - "Graphify Hooks Doc"
Cohesion: 0.29
Nodes (8): AuthUser Extractor, Role Model (seeker/owner/admin), AppError, frontend features/ directory, OpenAPI → TypeScript type generation pipeline, PaginatedResponse<T>, RBAC role guards, shared module

### Community 67 - "Auth Flow Wireframe"
Cohesion: 0.25
Nodes (7): name, typescript, overrides, openapi-typescript, private, type, version

### Community 68 - "CodeQL & Gitleaks Workflows"
Cohesion: 0.32
Nodes (4): Pagination(), PaginationProps, getPageItems(), PageItem

### Community 69 - "Docker Compose Files"
Cohesion: 0.43
Nodes (8): CI Job: Frontend Build, CI Job: Frontend OpenAPI Codegen, CI Job: Frontend Docker Build, CI Job: Frontend Install, CI Job: Frontend Lint, CI Job: Frontend Prettier, CI Job: Frontend Unit Tests, CI Job: Frontend TypeScript

### Community 70 - "Owner Request Email Templates"
Cohesion: 0.29
Nodes (7): AGENTS.md — graphify trigger instructions, CLAUDE.md — MyHouse project instructions, Architecture Invariants (modular monolith, handler→service→repository, AppError), Key Decisions Already Locked (OTP auth, role model, refresh token cookie, etc.), MCP Usage Policy (GitHub, PostgreSQL, Git, Context7, Filesystem, Sequential Thinking), Locked Stack Decision (Rust/Axum, React/TS, PostgreSQL, moka, Docker), mcp/.toolbox/tool.yaml — postgres-local MCP toolbox source

### Community 71 - "Pre-Tool-Use Hook Script"
Cohesion: 0.52
Nodes (4): MokaStore<K, V>, K, Option, V

### Community 72 - "Architecture Doc: Atomic Registration"
Cohesion: 0.29
Nodes (7): Multiple local subfolders (monorepo) flow, Multiple repos cross-repo graph merge, Step 0 - Clone GitHub repo(s), graphify.build.build_merge(), Replace-on-re-extract dedup fix (#1344/#1178), Step 0 - GitHub repos and multi-path merge, Step 0 - Clone GitHub repo(s) (Codex)

### Community 73 - "Feed/Detail Wireframe"
Cohesion: 0.29
Nodes (7): Part C - Merge AST + semantic into final extraction, Shrink-guard (#479), Step 4.5 - Graph health check, Step 4 - Build graph, cluster, analyze, generate outputs, Step 5 - Label communities, Step 6 - Generate Obsidian vault and HTML, Step B3 - Collect, cache, and merge

### Community 74 - "Frontend Root tsconfig"
Cohesion: 0.38
Nodes (7): admin module, media module, Notification stack (§3bis), notifications::service::send_xxx_email, synchronous best-effort email send, Testing strategy (§6), users module

### Community 75 - "README Project Overview"
Cohesion: 0.29
Nodes (7): Auth API endpoints (§4.1), auth module, OTP passwordless auth, owner request atomic submission, refresh token rotation, refresh_tokens table, registration_ticket

### Community 76 - "OpenAPI Codegen Binary"
Cohesion: 0.52
Nodes (5): FIELDS_BY_SERVER_NAME, maxLengthMessage(), ProfileFieldErrors, serverFieldError(), validate()

### Community 77 - "Architecture Doc: Auth Extractor & Roles"
Cohesion: 0.40
Nodes (6): react-typecrypt.md Rules File, Banned AI Marketing Words Rule, Embedded React/TypeScript Rules Content, Corrections vs Original React/TS Rule Files, README Writing Rules Skill, README Landing-Page Writing Principles

### Community 78 - "Architecture Doc: No-Proxy Media Rule"
Cohesion: 0.40
Nodes (6): --cluster-only, --update (incremental re-extraction), Interpreter guard for subcommands, For --update and --cluster-only (section pointer), --cluster-only (Codex), --update (incremental re-extraction) (Codex)

### Community 79 - "Architecture Doc: Search Vector Trigger"
Cohesion: 0.33
Nodes (5): CardPadding, CardProps, CardRadius, paddingClasses, radiusClasses

### Community 80 - "Architecture Doc: Refresh Token Cookie"
Cohesion: 0.50
Nodes (4): connect_db(), Error, PgPool, Result

### Community 81 - "Frontend Hooks ESLint Dep"
Cohesion: 0.40
Nodes (5): Honesty Rules, Stamped manifest files guard (#2015), Step 9 - Save manifest, update cost tracker, clean up, and report, Honesty Rules (Codex), Step 9 - Save manifest, update cost tracker, clean up (Codex)

### Community 82 - "Frontend Globals Dep"
Cohesion: 0.40
Nodes (5): owner_request_status enum, owner_requests table, OwnerRequestForm.tsx, PATCH /admin/owner-requests/:id, POST /owner-requests

### Community 83 - "Tailwind CSS Dep"
Cohesion: 0.67
Nodes (3): generate_otp_code(), generated_code_is_always_six_digits(), String

### Community 84 - "React DOM Types Dep"
Cohesion: 0.50
Nodes (4): Native CLAUDE.md integration (graphify claude install), git commit hook (graphify hook install), For the commit hook and native CLAUDE.md integration (section pointer), git commit hook (Codex)

### Community 85 - "typescript-eslint Dep"
Cohesion: 0.50
Nodes (4): Single Admin Account Bootstrap, Owner Request / Approval Flow, R-09: Single admin account operational risk, R-10: LIST-03 monthly reminder deferred to V2

### Community 86 - "Vite Dep"
Cohesion: 0.50
Nodes (4): mh-12-auth-flow.html wireframe, Email entry step page, OTP code verification step page, Profile completion step page

### Community 87 - "Admin Layout (Frontend)"
Cohesion: 0.50
Nodes (4): CodeQL Advanced Workflow, analyze job (rust + javascript-typescript matrix), Gitleaks Secret Scan Workflow, gitleaks job (secret scan)

### Community 88 - "Pre-Commit Hook Script"
Cohesion: 0.67
Nodes (3): Backend Docker Compose Config, Root Docker Compose, Frontend Docker Compose Config

### Community 89 - "Infra Module Root"
Cohesion: 0.67
Nodes (3): Owner Request Approved Email Template, Owner Request Received (Admin Notify) Email Template, Owner Request Rejected Email Template

### Community 91 - "Middleware Module Root"
Cohesion: 0.67
Nodes (3): Atomic Registration via POST /auth/register, OTP Passwordless Authentication, Registration Ticket (opaque UUID)

### Community 92 - "Admin Module Root"
Cohesion: 0.67
Nodes (3): mh-13-feed-detail.html wireframe, Public feed page (grid of cover photos), Listing detail page

### Community 94 - "Contact Module Root"
Cohesion: 0.67
Nodes (3): README.md — Project Overview and Setup, Conventional Commits convention, Trunk-based development branching strategy

## Ambiguous Edges - Review These
- `MyHouse Project Instructions (Agents)` → `React/TypeScript Rules (Agents)`  [AMBIGUOUS]
  .agents/rules/insrtruction-for-my-house.md · relation: references
- `README Writing Rules Skill` → `react-typecrypt.md Rules File`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with
- `README Writing Rules Skill` → `Embedded React/TypeScript Rules Content`  [AMBIGUOUS]
  .claude/skills/readme/SKILL.md · relation: shares_data_with

## Knowledge Gaps
- **290 isolated node(s):** `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem`, `postgres-mcp`, `DATABASE_URI` (+285 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **61 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `MyHouse Project Instructions (Agents)` and `React/TypeScript Rules (Agents)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `react-typecrypt.md Rules File`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **What is the exact relationship between `README Writing Rules Skill` and `Embedded React/TypeScript Rules Content`?**
  _Edge tagged AMBIGUOUS (relation: shares_data_with) - confidence is low._
- **Why does `AppError` connect `Auth Layout & Router` to `Owner Request API Client`, `User Profile DTOs & Validation`, `OTP Auth DTOs`, `Graphify Extraction Spec Doc`, `Frontend Package Scripts`, `App Cache (moka)`, `Owner Request HTTP Handler`, `App Config from Env`, `RequireAuth & Button Component`, `App Shell & Query Providers`, `Health Check Endpoint`, `Owner Request Document Classification`, `Owner/Listing Management Wireframe`, `Docker Compose Services & OTP Template`, `AppState Bootstrap`, `Technical Spec: Owner Request Deviations`, `OpenAPI Doc & Route Wiring`?**
  _High betweenness centrality (0.159) - this node is a cross-community bridge._
- **Why does `AppCacheProvider` connect `User Profile DTOs & Validation` to `Database Rules Doc`, `Pre-Tool-Use Hook Script`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **Why does `Mailer` connect `Root Layout & Site Header` to `User Profile DTOs & Validation`, `MCP Server Config`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
- **What connects `@modelcontextprotocol/server-github`, `GITHUB_PERSONAL_ACCESS_TOKEN`, `@modelcontextprotocol/server-filesystem` to the rest of the system?**
  _290 weakly-connected nodes found - possible documentation gaps or missing edges._